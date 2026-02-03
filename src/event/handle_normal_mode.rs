use crate::app::{App, CommandStatus};
use crate::process::ProcessManager;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc::UnboundedSender;

use crate::event::AppEvent;

pub async fn handle_normal_mode(
    app: &mut App,
    process_manager: &mut ProcessManager,
    key: KeyEvent,
    event_tx: UnboundedSender<AppEvent>,
) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Char('a') => app.change_adding_mode(),
        KeyCode::Char('j') | KeyCode::Down => app.select_next_command(),
        KeyCode::Char('k') | KeyCode::Up => app.select_previous_command(),
        KeyCode::Enter => {
            if let Some(command) = app.get_selected_command() {
                match command.status() {
                    CommandStatus::Running => {
                        process_manager.stop(command.id()).await?;
                    }
                    CommandStatus::Stopped | CommandStatus::Error(_) => {
                        match process_manager.spawn(command, event_tx.clone()).await {
                            Ok(pid) => {
                                app.mark_command_run(command.id(), pid);
                            }
                            Err(_e) => {}
                        }
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
