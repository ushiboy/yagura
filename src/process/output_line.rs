use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct OutputLine {
    timestamp: DateTime<Local>,
    content: String,
}

impl OutputLine {
    pub fn new(content: String) -> Self {
        OutputLine {
            timestamp: Local::now(),
            content,
        }
    }
    pub fn timestamp(&self) -> &DateTime<Local> {
        &self.timestamp
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_output_line_new() {
        let content = "Test output line".to_string();
        let output_line = OutputLine::new(content.clone());
        assert_eq!(output_line.content(), content);
    }
}
