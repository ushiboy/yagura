// The UI state of the application
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIState {
    // The state of the command list UI component.
    command_list: CommandList,
    // The state of the adding command form UI component.
    adding_command_form: AddingCommandForm,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            command_list: CommandList::new(),
            adding_command_form: AddingCommandForm::new(),
        }
    }

    pub fn selected_command_index(&self) -> Option<usize> {
        self.command_list.selected_command_index()
    }

    pub fn set_selected_index(&mut self, index: usize) {
        self.command_list.set_selected_index(index);
    }

    pub fn clear_selection(&mut self) {
        self.command_list.clear_selection();
    }

    pub fn command_list_scroll_offset(&self) -> usize {
        self.command_list.scroll_offset()
    }

    pub fn set_command_list_scroll_offset(&mut self, offset: usize) {
        self.command_list.set_scroll_offset(offset);
    }

    pub fn adding_command_form(&self) -> &AddingCommandForm {
        &self.adding_command_form
    }

    pub fn adding_command_form_mut(&mut self) -> &mut AddingCommandForm {
        &mut self.adding_command_form
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
