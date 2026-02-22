use ratatui::Frame;

use super::FrameContext;
use super::add_command_dialog;
use super::command_list;
use super::command_list_area_help_bar;
use super::delete_command_dialog;
use super::output_area;
use super::output_area_help_bar;
use crate::model::AppMode;

use super::super::model::App;

pub fn render(frame: &mut Frame, app: &App, frame_context: &FrameContext) {
    command_list::render(frame, frame_context.command_list_area, app);

    command_list_area_help_bar::render(frame, frame_context.command_list_help_bar_area, app);

    output_area::render(frame, frame_context.output_area, app);

    output_area_help_bar::render(frame, frame_context.help_bar_area, app);

    match app.mode() {
        AppMode::Normal => {
            // ignore
        }
        AppMode::AddingCommand => {
            add_command_dialog::render(frame, app);
        }
        AppMode::DeletingCommand => {
            delete_command_dialog::render(frame, app);
        }
    }
}
