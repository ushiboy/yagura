pub struct Form {
    command_input: String,
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
        }
    }

    pub fn command_input(&self) -> &str {
        &self.command_input
    }

    pub fn push_char(&mut self, c: char) {
        self.command_input.push(c);
    }

    pub fn pop_char(&mut self) {
        self.command_input.pop();
    }
}
