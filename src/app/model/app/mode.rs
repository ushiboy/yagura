use super::{App, AppMode};

impl App {
    pub fn change_adding_mode(&mut self) {
        self.mode = AppMode::AddingCommand;
    }

    pub fn change_normal_mode(&mut self) {
        self.mode = AppMode::Normal;
    }
}
