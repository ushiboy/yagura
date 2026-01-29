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
        KeyCode::Tab => {
            app.form_mut().toggle_focused_input();
        }
        KeyCode::Enter => {
            let form = app.form();
            let command_text = form.command_input().to_string();
            let working_dir = if form.working_dir_input().is_empty() {
                None
            } else {
                Some(form.working_dir_input().to_string())
            };
            add_command(app, command_text, working_dir);
            app.change_normal_mode();
        }
        _ => { /* Ignore other keys */ }
    }

    Ok(())
}
