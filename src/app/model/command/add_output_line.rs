use super::super::OutputLine;
use super::Command;

impl Command {
    pub fn add_output_line(&mut self, line: OutputLine) {
        self.output_buffer.push_line(line);
    }
}
