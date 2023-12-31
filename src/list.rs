use core::fmt;

use ratatui::{widgets::StatefulWidget, style::Style};

#[derive(Clone, Copy, PartialEq)]
pub enum MarkState {
    Unmarked,
    Marked,
}

pub enum MarkAction {
    Mark,
    Unmark,
    Toggle,
}

// Discussion: This would be awesome as a trait which I could just attach.
impl MarkState {
    pub fn new() -> Self {
        MarkState::Unmarked
    }
    pub fn is(&self, state: &MarkState) -> bool {
        self == state
    }

    /* Discussion: What is better? */
    pub fn act(&mut self, action: &MarkAction) {
        *self = match action {
            MarkAction::Mark => MarkState::Marked,
            MarkAction::Unmark => MarkState::Unmarked,
            MarkAction::Toggle => match self {
                MarkState::Unmarked => MarkState::Marked,
                MarkState::Marked => MarkState::Unmarked,
            }
        };
    }
    /* OR */
    #[deprecated(since="0.1.0", note="please use `act` instead")]
    pub fn unmark(&self) -> Self {
        MarkState::Unmarked
    }
    #[deprecated(since="0.1.0", note="please use `act` instead")]
    pub fn mark(&self) -> Self {
        MarkState::Marked
    }
    #[deprecated(since="0.1.0", note="please use `act` instead")]
    pub fn toggle(&self) -> Self {
        match self {
            MarkState::Unmarked => MarkState::Marked,
            MarkState::Marked => MarkState::Unmarked,
        }
    }
}

impl fmt::Display for MarkState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                MarkState::Unmarked => "[ ]",
                MarkState::Marked   => "[x]",
            }
        )
    }
}

#[cfg(test)]
mod markable_state_tests {
    use super::*;

    #[test]
    fn newable() {
        let state = MarkState::new();
        assert!(state == MarkState::Unmarked);
    }

    #[test]
    fn comparable() {
        assert!(MarkState::Marked.is(&MarkState::Marked) == true);
        assert!(MarkState::Marked.is(&MarkState::Unmarked) == false);
        assert!(MarkState::Unmarked.is(&MarkState::Marked) == false);
        assert!(MarkState::Unmarked.is(&MarkState::Unmarked) == true);
    }

    #[test]
    fn markable() {
        let mut state = MarkState::Unmarked;
        state.act(&MarkAction::Mark);
        assert!(state == MarkState::Marked);
        state.act(&MarkAction::Mark);
        assert!(state == MarkState::Marked);
    }

    #[test]
    fn unmarkable() {
        let mut state = MarkState::Marked;
        state.act(&MarkAction::Unmark);
        assert!(state == MarkState::Unmarked);
        state.act(&MarkAction::Unmark);
        assert!(state == MarkState::Unmarked);
    }

    #[test]
    fn togglable() {
        let mut state = MarkState::Unmarked;
        state.act(&MarkAction::Toggle);
        assert!(state == MarkState::Marked);
        state.act(&MarkAction::Toggle);
        assert!(state == MarkState::Unmarked);
    }
    #[test]
    fn displayable() {
        assert!(MarkState::Unmarked.to_string() == "[ ]");
        assert!(MarkState::Marked.to_string() == "[x]");
    }
}

#[derive(Clone, Copy)]
pub struct Item<T> {
    pub mark_state: MarkState,
    pub data: T,
}

/**
 * Discussion: I really do not want to implement that boilerplate twice. Especially with tests.
 * I found a delegate macro, but is this da wae of composition in rust???
 */
/*
impl<T> Item<T> {
    pub fn is(&self, state: MarkStatus) -> bool {
        self.mark_status == state
    }
    pub fn is_marked(&self) -> bool {
        self.is(MarkStatus::Marked)
    }
    pub fn is_unmarked(&self) -> bool {
        self.is(MarkStatus::Unmarked)
    }
    */
    /*
    pub fn mark(&self) {
        self.mark_status.mark();
    }
    pub fn unmark(&self) {
        self.mark_status.unmark();
    }
}
*/

impl<T> From<T> for Item<T> {
    fn from(value: T) -> Self {
        Item {
            mark_state: MarkState::new(),
            data: value
        }
    }
}

impl<T> fmt::Display for Item<T> where
    T: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {}",
            self.mark_state,
            self.data,
        )
    }
}

#[derive(Clone)]
pub struct List<T> {
    items: Vec<Item<T>>,
}

/**
 * Discussion:
 *   This API certainly sucks and probably is not "idiomatic Rust", but how to conciliate?
 *   - Return Slice/&[] or Iter or just a pointer to Vec?
 *   - How to input data if you want to act on several elements? Slice? Range? Offset + Size?
 *   - Should the return value of get_displayed return a result and throw errors?
 */
impl<T> List<T>  {
    pub fn new(items: Vec<T>) -> List<T> {
        List {
            items: items.into_iter().map(|element| element.into()).collect(),
        }
    }

    pub fn act(&mut self, action: &MarkAction, range: &[usize]) {
        for index in range {
            let item = self.items.get_mut(*index).unwrap();
            item.mark_state.act(action);
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn all(&self) -> &Vec<Item<T>> {
        &self.items
    }

    pub fn by_state(&self, state: &MarkState) -> Vec<&Item<T>> {
        self.items.iter().enumerate().filter_map(|(_index, element)|
            if element.mark_state.is(&state) {
                Some(element)
            } else {
                None
            }
        ).collect::<Vec<_>>()
    }

    fn displayed(&self, start: usize, len: usize) -> &[Item<T>] {
        if start > self.items.len() {
            return &[];
        }
        if start + len > self.items.len() {
            return self.items.get(start..self.items.len()).unwrap();
        }
        self.items.get(start..(start + len)).unwrap()
    }
}

impl<T> fmt::Display for List<T> where
    T: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.all() {
            write!(
                formatter,
                "{}\n",
                item,
            )?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct DisplayState {
    pub offset: usize,
    pub current: usize,
    normal: Style,
    highlight: Style,
}

pub enum ListStyle {
    Highlight { style: Style },
    Normal { style: Style },
}

pub enum ListMoves {
    Up,
    Down,
}

impl DisplayState {
    pub fn act(&mut self, movement: ListMoves) {
        match movement {
            ListMoves::Up => self.current -= 1,
            ListMoves::Down => self.current += 1,
        }
    }
    pub fn style(&mut self, style: ListStyle) {
        match style {
            ListStyle::Normal { style } => self.normal = style,
            ListStyle::Highlight { style } => self.highlight = style,
        }
    }
}

impl<T> StatefulWidget for List<T> where
    T: fmt::Display
{
    type State = DisplayState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let height = area.height as usize;
        let width = area.width as usize;
        let border = "─".repeat(width);

        buf.set_string(0, 1,&border, state.normal);
        for(index, item) in self.displayed(state.offset, height - 4).iter().enumerate() {
            let style = match state.current != state.offset + index {
                true => state.normal,
                false => state.highlight,
            };

            buf.set_string(area.x, area.y + 2 + index as u16, item.to_string(), style);
        }
        buf.set_string(0, area.height - 2, &border, state.normal);
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    #[test]
    fn newable() {
        let list: List<i32> = List::new(vec![1, 2, 3]);
        assert!(list.len() == 3);
    }
}

