use crate::app::App;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub async fn handle_adding_command_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => app.change_normal_mode(),
        KeyCode::Char(c) => {
            app.form_mut().push_char(c);
        }
        KeyCode::Backspace => {
            app.form_mut().pop_char();
        }
        _ => { /* Ignore other keys */ }
    }

    Ok(())
}
