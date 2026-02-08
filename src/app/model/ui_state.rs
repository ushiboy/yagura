#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIState {
    pub command_list: CommandList,
    pub adding_command_form: AddingCommandForm,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            command_list: CommandList::new(),
            adding_command_form: AddingCommandForm::new(),
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

mod command_list;
pub mod form;
use command_list::CommandList;
pub use form::AddingCommandForm;
