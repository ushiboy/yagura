pub struct Form {
    command_input: String,
    working_dir_input: String,
    focused_input: FocusedInput,
}

pub enum FocusedInput {
    Command,
    WorkingDir,
}

impl Default for Form {
    fn default() -> Self {
        Self::new()
    }
}

impl Form {
    pub fn new() -> Self {
        Self {
            command_input: String::new(),
            working_dir_input: String::new(),
            focused_input: FocusedInput::Command,
        }
    }

    pub fn command_input(&self) -> &str {
        &self.command_input
    }

    pub fn working_dir_input(&self) -> &str {
        &self.working_dir_input
    }

    pub fn focused_input(&self) -> &FocusedInput {
        &self.focused_input
    }

    pub fn toggle_focused_input(&mut self) {
        self.focused_input = match self.focused_input {
            FocusedInput::Command => FocusedInput::WorkingDir,
            FocusedInput::WorkingDir => FocusedInput::Command,
        };
    }

    pub fn push_char(&mut self, c: char) {
        match self.focused_input {
            FocusedInput::Command => self.command_input.push(c),
            FocusedInput::WorkingDir => self.working_dir_input.push(c),
        }
    }

    pub fn pop_char(&mut self) {
        match self.focused_input {
            FocusedInput::Command => self.command_input.pop(),
            FocusedInput::WorkingDir => self.working_dir_input.pop(),
        };
    }
}
