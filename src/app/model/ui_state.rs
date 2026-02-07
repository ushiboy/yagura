#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIState {
    pub command_list: CommandList,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            command_list: CommandList::new(),
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

mod command_list;
use command_list::CommandList;
