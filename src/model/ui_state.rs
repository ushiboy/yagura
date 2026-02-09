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

pub mod adding_command_form;
mod command_list;
pub use adding_command_form::AddingCommandForm;
use command_list::CommandList;
