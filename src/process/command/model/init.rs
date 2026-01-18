use super::Command;
use uuid::{NoContext, Timestamp, Uuid};

impl Command {
    pub fn new(command: String) -> Self {
        let ts = Timestamp::now(NoContext);
        Self {
            id: Uuid::new_v7(ts),
            command,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new() {
        let command = Command::new("sleep".to_string());
        assert_eq!(command.command(), "sleep");
    }
}
