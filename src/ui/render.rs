use ratatui::Frame;

use crate::app::AppMode;
use crate::ui::help_bar;

use super::add_command_dialog;
use super::command_list;
use super::output_area;

use super::split_layout::split_layout;

use super::super::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = split_layout(frame);

    command_list::render(frame, chunks[0], app);

    output_area::render(frame, chunks[1], app);

    help_bar::render(frame, chunks[2], app);

    match app.mode() {
        AppMode::Normal => {
            // ignore
        }
        AppMode::AddingCommand => {
            add_command_dialog::render(frame, app);
        }
    }
}
