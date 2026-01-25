use crate::app::App;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub async fn handle_adding_command_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.code == KeyCode::Esc { app.change_normal_mode() }

    Ok(())
}
