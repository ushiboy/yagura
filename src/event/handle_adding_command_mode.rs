use crate::model::App;
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
        KeyCode::Tab => {
            app.form_mut().toggle_focused_input();
        }
        KeyCode::Enter => {
            app.confirm_add_command();
        }
        _ => { /* Ignore other keys */ }
    }

    Ok(())
}
