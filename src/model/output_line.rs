use chrono::{DateTime, Local};

// Represents a single line of output with a timestamp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputLine {
    // Timestamp when the output line was created.
    timestamp: DateTime<Local>,
    // The actual content of the output line.
    content: String,
}

impl OutputLine {
    // Creates a new OutputLine with the current timestamp and given content.
    pub fn new(content: impl Into<String>) -> Self {
        OutputLine {
            timestamp: Local::now(),
            content: content.into(),
        }
    }

    // Returns the timestamp of the output line.
    pub fn timestamp(&self) -> &DateTime<Local> {
        &self.timestamp
    }

    // Returns the content of the output line.
    pub fn content(&self) -> &str {
        &self.content
    }

    // Returns the content of the output line with ANSI escape codes stripped.
    pub fn plain_text(&self) -> String {
        String::from_utf8_lossy(&strip_ansi_escapes::strip(self.content())).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_output_line_new() {
        let content = "Test output line";

        let output_line = OutputLine::new(content);

        assert_eq!(output_line.content(), content);
        assert!(output_line.timestamp() <= &Local::now());
    }

    #[test]
    fn test_output_line_plain_text() {
        let content = "Test \x1b[31moutput\x1b[0m line";
        let output_line = OutputLine::new(content);

        assert_eq!(output_line.plain_text(), "Test output line");
    }
}
