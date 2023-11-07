use core::fmt;

#[derive(PartialEq)]
pub enum MarkableState {
    Unmarked,
    Marked,
}

pub enum MarkableAction {
    Mark,
    Unmark,
    Toggle,
}

// Discussion: This would be awesome as a trait which I could just attach.
impl MarkableState {
    pub fn new() -> Self {
        MarkableState::Unmarked
    }
    pub fn is(&self, state: &MarkableState) -> bool {
        self == state
    }

    /* Discussion: What is better? */
    pub fn act(&mut self, action: &MarkableAction) {
        *self = match action {
            MarkableAction::Mark => MarkableState::Marked,
            MarkableAction::Unmark => MarkableState::Unmarked,
            MarkableAction::Toggle => match self {
                MarkableState::Unmarked => MarkableState::Marked,
                MarkableState::Marked => MarkableState::Unmarked,
            }
        };
    }
    /* OR */
    #[deprecated(since="0.1.0", note="please use `act` instead")]
    pub fn unmark(&self) -> Self {
        MarkableState::Unmarked
    }
    #[deprecated(since="0.1.0", note="please use `act` instead")]
    pub fn mark(&self) -> Self {
        MarkableState::Marked
    }
    #[deprecated(since="0.1.0", note="please use `act` instead")]
    pub fn toggle(&self) -> Self {
        match self {
            MarkableState::Unmarked => MarkableState::Marked,
            MarkableState::Marked => MarkableState::Unmarked,
        }
    }
}

impl fmt::Display for MarkableState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                MarkableState::Unmarked => "[ ]",
                MarkableState::Marked   => "[x]",
            }
        )
    }
}

#[cfg(test)]
mod markable_state_tests {
    use super::*;

    #[test]
    fn newable() {
        let state = MarkableState::new();
        assert!(state == MarkableState::Unmarked);
    }

    #[test]
    fn comparable() {
        assert!(MarkableState::Marked.is(&MarkableState::Marked) == true);
        assert!(MarkableState::Marked.is(&MarkableState::Unmarked) == false);
        assert!(MarkableState::Unmarked.is(&MarkableState::Marked) == false);
        assert!(MarkableState::Unmarked.is(&MarkableState::Unmarked) == true);
    }

    #[test]
    fn markable() {
        let mut state = MarkableState::Unmarked;
        state.act(&MarkableAction::Mark);
        assert!(state == MarkableState::Marked);
        state.act(&MarkableAction::Mark);
        assert!(state == MarkableState::Marked);
    }

    #[test]
    fn unmarkable() {
        let mut state = MarkableState::Marked;
        state.act(&MarkableAction::Unmark);
        assert!(state == MarkableState::Unmarked);
        state.act(&MarkableAction::Unmark);
        assert!(state == MarkableState::Unmarked);
    }

    #[test]
    fn togglable() {
        let mut state = MarkableState::Unmarked;
        state.act(&MarkableAction::Toggle);
        assert!(state == MarkableState::Marked);
        state.act(&MarkableAction::Toggle);
        assert!(state == MarkableState::Unmarked);
    }
    #[test]
    fn displayable() {
        assert!(MarkableState::Unmarked.to_string() == "[ ]");
        assert!(MarkableState::Marked.to_string() == "[x]");
    }
}

pub struct Item<T> {
    pub mark_state: MarkableState,
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
            mark_state: MarkableState::new(),
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

    pub fn get_by_state(&self, state: &MarkableState) -> Vec<&Item<T>>{
        self.elements.iter().enumerate().filter_map(|(_index, element)|
            if element.mark_state.is(&state) {
                Some(element)
            } else {
                None
            }
        ).collect::<Vec<_>>()
    }

    pub fn act(&mut self, action: &MarkableAction, range: &[usize]) {
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

