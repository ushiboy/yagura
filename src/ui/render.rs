use ratatui::Frame;

use super::command_list;
use super::output_area;

use super::split_layout::split_layout;

use super::super::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = split_layout(frame);

    command_list::render(frame, chunks[0], app);

    output_area::render(frame, chunks[1], app);
}
