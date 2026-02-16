// The UI state of the application
#[derive(Debug, PartialEq, Eq)]
pub struct UIState {
    // The state of the command list UI component.
    command_list: CommandList,
    // The state of the adding command form UI component.
    adding_command_form: AddingCommandForm,
    // The state of the command log UI component.
    command_log: CommandLog,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            command_list: CommandList::new(),
            adding_command_form: AddingCommandForm::new(),
            command_log: CommandLog::new(),
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

    pub fn get_command_log_offset(&self, command_id: Uuid) -> Option<usize> {
        self.command_log.get_offset(&command_id)
    }

    pub fn set_command_log_offset(&mut self, command_id: Uuid, offset: usize) {
        self.command_log.set_offset(command_id, offset);
    }

    pub fn remove_command_log_offset(&mut self, command_id: Uuid) {
        self.command_log.remove_offset(&command_id);
    }

    pub fn adding_command_form(&self) -> &AddingCommandForm {
        &self.adding_command_form
    }

    pub fn adding_command_form_mut(&mut self) -> &mut AddingCommandForm {
        &mut self.adding_command_form
    }

    pub fn toggle_command_log_timestamp_visibility(&mut self) {
        self.command_log.toggle_timestamp_visibility();
    }

    pub fn command_log_timestamp_visibility(&self) -> bool {
        self.command_log.timestamp_visibility()
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

pub mod adding_command_form;
mod command_list;
mod command_log;
pub use adding_command_form::{AddingCommandForm, FocusedInput};
use command_list::CommandList;
pub use command_log::CommandLog;
use uuid::Uuid;
