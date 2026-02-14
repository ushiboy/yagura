use crate::model::{App, AppMode};
use crate::process::ProcessManager;
use crate::ui::ViewportMetrics;
use anyhow::Result;
use crossterm::event::KeyEvent;
use tokio::sync::mpsc::Sender;

use super::{
    AppEvent, handle_adding_command_mode::handle_adding_command_mode,
    handle_deleting_command_mode::handle_deleting_command_mode,
    handle_normal_mode::handle_normal_mode,
};

pub async fn handle_key_event(
    app: &mut App,
    process_manager: &mut ProcessManager,
    key: KeyEvent,
    event_tx: Sender<AppEvent>,
    viewport_metrics: ViewportMetrics,
) -> Result<()> {
    match app.mode() {
        AppMode::Normal => {
            handle_normal_mode(app, process_manager, key, event_tx, viewport_metrics).await?
        }
        AppMode::AddingCommand => handle_adding_command_mode(app, key).await?,
        AppMode::DeletingCommand => handle_deleting_command_mode(app, process_manager, key).await?,
    }
    Ok(())
}
