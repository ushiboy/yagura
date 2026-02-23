use uuid::Uuid;

use super::{App, Command};
use crate::process::{ExitCode, ProcessId};

impl App {
    // Adds a new command to the application's command list.
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    // Removes a specific command from the application's command list.
    pub fn remove_command_by_id(&mut self, command_id: Uuid) {
        self.commands.retain(|c| c.id() != command_id);
    }

    // Retrieves a mutable reference to a command by its unique identifier.
    pub fn get_command_mut_by_id(&mut self, command_id: Uuid) -> Option<&mut Command> {
        self.commands.iter_mut().find(|cmd| cmd.id() == command_id)
    }

    // Marks a command as running by setting its process ID.
    pub fn mark_command_run(&mut self, command_id: Uuid, pid: ProcessId) {
        if let Some(command) = self.get_command_mut_by_id(command_id) {
            command.mark_running(pid);
        }
    }

    // Marks a command as exited by setting its exit code.
    pub fn mark_command_exit(&mut self, command_id: Uuid, exit_code: ExitCode) {
        if let Some(command) = self.get_command_mut_by_id(command_id) {
            command.mark_exit(exit_code);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::CommandStatus;

    use super::*;

    #[test]
    fn test_add_command() {
        let mut app = App::new();
        let command = Command::new("ls -la");

        app.add_command(command);

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "ls -la");
    }

    #[test]
    fn test_remove_command_by_id() {
        let mut app = App::new();
        let command1 = Command::new("ls -la");
        let command2 = Command::new("pwd");
        let command3 = Command::new("whoami");
        app.add_command(command1.clone());
        app.add_command(command2.clone());
        app.add_command(command3.clone());

        app.remove_command_by_id(command1.id());

        assert_eq!(app.commands().len(), 2);
        assert_eq!(app.commands()[0].command(), "pwd");
        assert_eq!(app.commands()[1].command(), "whoami");
    }

    #[test]
    fn test_get_command_mut_by_id() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        let command_id = command.id();
        app.add_command(command);

        let cmd_mut = app.get_command_mut_by_id(command_id);
        assert!(cmd_mut.is_some());
        assert_eq!(cmd_mut.unwrap().command(), "ls -la");
    }

    #[test]
    fn test_get_command_mut_by_id_with_non_existent_id() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        app.add_command(command);

        let non_existent_id = Uuid::now_v7();
        let cmd_none = app.get_command_mut_by_id(non_existent_id);
        assert!(cmd_none.is_none());
    }

    #[test]
    fn test_mark_command_run() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        let command_id = command.id();
        app.add_command(command);

        let pid = ProcessId(12345);
        app.mark_command_run(command_id, pid);

        let cmd_mut = app.get_command_mut_by_id(command_id).unwrap();
        assert_eq!(cmd_mut.pid(), Some(pid));
    }

    #[test]
    fn test_mark_command_exit() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        let command_id = command.id();
        app.add_command(command);

        let exit_code = ExitCode::Code(0);
        app.mark_command_exit(command_id, exit_code);

        let cmd_mut = app.get_command_mut_by_id(command_id).unwrap();
        assert_eq!(cmd_mut.status(), &CommandStatus::Stopped);
    }
}
