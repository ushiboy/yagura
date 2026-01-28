use crate::app::{App, add_command};
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
        KeyCode::Enter => {
            let form = app.form();
            let command_text = form.command_input().to_string();
            add_command(app, command_text);
            app.change_normal_mode();
        }
        _ => { /* Ignore other keys */ }
    }

    Ok(())
}
