pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:.<width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
