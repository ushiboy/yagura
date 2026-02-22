use ratatui::{Frame, layout::Rect};

use crate::ui::split_layout::split_layout;

#[derive(Debug, Clone, Default)]
pub struct FrameContext {
    pub command_list_area: Rect,
    pub command_list_help_bar_area: Rect,
    pub output_area: Rect,
    pub help_bar_area: Rect,
}

pub fn build_frame_context(frame: &mut Frame) -> FrameContext {
    let layout = split_layout(frame);

    FrameContext {
        command_list_area: layout[0],
        command_list_help_bar_area: layout[1],
        output_area: layout[2],
        help_bar_area: layout[3],
    }
}
