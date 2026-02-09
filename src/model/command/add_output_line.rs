use super::super::OutputLine;
use super::Command;

impl Command {
    // Adds a new output line to the command's output buffer.
    pub fn add_output_line(&mut self, line: OutputLine) {
        self.output_buffer.push_line(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_output_line() {
        let mut command = Command::new("echo 'Hello, World!'");
        let line = OutputLine::new("Hello, World!");

        command.add_output_line(line.clone());

        assert_eq!(command.output_buffer().lines().len(), 1);
        assert_eq!(command.output_buffer().lines()[0], line);
    }
}
