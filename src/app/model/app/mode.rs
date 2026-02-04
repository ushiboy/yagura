use super::{App, AppMode};

impl App {
    pub fn change_adding_mode(&mut self) {
        self.form_mut().clear();
        self.mode = AppMode::AddingCommand;
    }

    pub fn change_deleting_mode(&mut self) {
        if self.selected_command_index().is_none() {
            return;
        }
        self.mode = AppMode::DeletingCommand;
    }

    pub fn change_normal_mode(&mut self) {
        self.form_mut().clear();
        self.mode = AppMode::Normal;
    }
}
