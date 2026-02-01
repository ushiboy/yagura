// Model for the adding command form
pub struct Form {
    // Input for the command to run
    command_input: String,
    // Input for the working directory to run the command in
    working_dir_input: String,
    // Currently focused input field
    focused_input: FocusedInput,
}

// Enum representing which input field is focused
pub enum FocusedInput {
    // Input for the command to run
    Command,
    // Input for the working directory to run the command in
    WorkingDir,
}

impl Default for Form {
    fn default() -> Self {
        Self::new()
    }
}

impl Form {
    // Create a new Form with default values
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

    // Toggle the focused input field between command and working directory
    pub fn toggle_focused_input(&mut self) {
        self.focused_input = match self.focused_input {
            FocusedInput::Command => FocusedInput::WorkingDir,
            FocusedInput::WorkingDir => FocusedInput::Command,
        };
    }

    // Push a character to the currently focused input field
    pub fn push_char(&mut self, c: char) {
        match self.focused_input {
            FocusedInput::Command => self.command_input.push(c),
            FocusedInput::WorkingDir => self.working_dir_input.push(c),
        }
    }

    // Pop a character from the currently focused input field
    pub fn pop_char(&mut self) {
        match self.focused_input {
            FocusedInput::Command => self.command_input.pop(),
            FocusedInput::WorkingDir => self.working_dir_input.pop(),
        };
    }

    // Clear both input fields and reset focus to command input
    pub fn clear(&mut self) {
        self.command_input.clear();
        self.working_dir_input.clear();
        self.focused_input = FocusedInput::Command;
    }

    #[cfg(test)]
    pub(super) fn set_command_input(&mut self, input: impl Into<String>) {
        self.command_input = input.into();
    }

    #[cfg(test)]
    pub(super) fn set_working_dir_input(&mut self, input: impl Into<String>) {
        self.working_dir_input = input.into();
    }

    #[cfg(test)]
    pub(super) fn set_focused_input(&mut self, focused: FocusedInput) {
        self.focused_input = focused;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_default() {
        let form = Form::default();

        assert_eq!(form.command_input(), "");
        assert_eq!(form.working_dir_input(), "");
        matches!(form.focused_input(), FocusedInput::Command);
    }

    #[test]
    fn test_form_push_char() {
        let mut form = Form::new();

        form.push_char('c');

        assert_eq!(form.command_input(), "c");
    }

    #[test]
    fn test_form_pop_char() {
        let mut form = Form::new();
        form.set_command_input("abc");

        form.pop_char();

        assert_eq!(form.command_input(), "ab");
    }

    #[test]
    fn test_form_toggle_focused_input() {
        let mut form = Form::new();
        form.set_focused_input(FocusedInput::Command);

        form.toggle_focused_input();

        matches!(form.focused_input(), FocusedInput::WorkingDir);
    }

    #[test]
    fn test_form_clear() {
        let mut form = Form::new();
        form.set_command_input("cmd");
        form.set_working_dir_input("/path/to/dir");
        form.set_focused_input(FocusedInput::WorkingDir);

        form.clear();

        assert_eq!(form.command_input(), "");
        assert_eq!(form.working_dir_input(), "");
        matches!(form.focused_input(), FocusedInput::Command);
    }
}
