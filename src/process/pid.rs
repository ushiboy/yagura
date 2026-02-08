#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessId(pub u32);

impl std::fmt::Display for ProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_creation() {
        let pid = ProcessId(1234);
        assert_eq!(pid.0, 1234);
    }

    #[test]
    fn test_pid_display() {
        let pid = ProcessId(5678);
        assert_eq!(format!("{}", pid), "5678");
    }
}
