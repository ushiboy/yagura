use unicode_width::UnicodeWidthChar;

pub fn truncate_string(s: &str, max_len: usize) -> String {
    let mut width = 0;
    let mut result = String::new();

    for c in s.chars() {
        let char_width = c.width().unwrap_or(0);

        if width + char_width > max_len.saturating_sub(3) {
            result.push_str("...");
            return format!("{:.<width$}", result, width = max_len);
        }
        result.push(c);
        width += char_width;
    }

    format!("{:.<width$}", result, width = max_len)
}
