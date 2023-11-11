use core::fmt;

#[derive(PartialEq)]
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

impl<T> fmt::Display for Item<T>
where
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

pub struct List<T> {
    elements: Vec<Item<T>>
}

impl<T> List<T>  {
    pub fn new(elements: Vec<T>) -> List<T> {
        List {
            elements: elements.into_iter().map(|element| element.into()).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn get_all(&self) -> &Vec<Item<T>> {
        &self.elements
    }

    pub fn get_by_state(&self, state: &MarkState) -> Vec<&Item<T>>{
        self.elements.iter().enumerate().filter_map(|(_index, element)|
            if element.mark_state.is(&state) {
                Some(element)
            } else {
                None
            }
        ).collect::<Vec<_>>()
    }

    pub fn act(&mut self, action: &MarkAction, range: &[usize]) {
        for index in range {
            let item = self.elements.get_mut(*index).unwrap();
            item.mark_state.act(action);
        }
    }
}

impl<T> fmt::Display for List<T>
where
    T: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.get_all() {
            write!(
                formatter,
                "{}\n",
                item,
            )?;
        }
        Ok(())
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

