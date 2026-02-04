use crate::app::{App, CommandStatus};
use crate::process::ProcessManager;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub async fn handle_deleting_command_mode(
    app: &mut App,
    process_manager: &mut ProcessManager,
    key: KeyEvent,
) -> Result<()> {
    match key.code {
        KeyCode::Char('y') => {
            if let Some(command) = app.get_selected_command()
                && command.status() == &CommandStatus::Running
            {
                process_manager.stop(command.id()).await?;
            }
            app.delete_selected_command();
            app.change_normal_mode();
        }
        KeyCode::Char('n') | KeyCode::Esc => app.change_normal_mode(),
        _ => { /* Ignore other keys */ }
    }

    Ok(())
}
