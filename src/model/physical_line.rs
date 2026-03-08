use ansi_to_tui::IntoText;
use chrono::{DateTime, Local};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};

// Calculates the number of physical lines needed to display the given text within the specified viewport width.
pub fn calculate_physical_lines(text: &str, viewport_width: usize) -> usize {
    if viewport_width == 0 {
        return 0;
    }

    let content = text.into_text().unwrap_or_else(|_| Text::from(text));
    let paragraph = Paragraph::new(content).wrap(Wrap { trim: true });
    paragraph.line_count(viewport_width as u16)
}

// Formats a string with an optional timestamp prefix. If `show_timestamp` is true, the timestamp is formatted as "[HH:MM:SS] " and prepended to the string.
pub fn format_str(timestamp: &DateTime<Local>, str: &str, show_timestamp: bool) -> String {
    if show_timestamp {
        format!("[{}] {}", timestamp.format("%H:%M:%S"), str)
    } else {
        str.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_calculate_physical_lines_empty_string() {
        assert_eq!(calculate_physical_lines("", 80), 1);
    }

    #[test]
    fn test_calculate_physical_lines_single_line() {
        assert_eq!(calculate_physical_lines("Hello, world!", 80), 1);
    }

    #[test]
    fn test_calculate_physical_lines_wrapping() {
        let text = "a".repeat(200);

        let physical_lines = calculate_physical_lines(&text, 80);
        assert_eq!(physical_lines, 3);
    }

    #[test]
    fn test_calculate_physical_lines_zero_width() {
        assert_eq!(calculate_physical_lines("Hello", 0), 0);
    }

    #[test]
    fn test_format_str_with_timestamp() {
        let timestamp = Local::now();
        let formated_timestamp = timestamp.format("%H:%M:%S").to_string();

        let result = format_str(&timestamp, "test message", true);

        let expected = format!("[{}] test message", formated_timestamp);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_str_without_timestamp() {
        let timestamp = Local::now();

        let result = format_str(&timestamp, "test message", false);

        assert_eq!(result, "test message");
    }
}
