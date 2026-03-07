use crate::{model::App, ui::frame_context::FrameContext};
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: Rect, app: &App, frame_context: &FrameContext) {
    let scroll_offset = app.get_command_log_offset();

    let output_line_count = app
        .get_selected_command()
        .map_or(0, |cmd| cmd.output_buffer().line_length());

    let debug_text = vec![Line::from(format!(
        "[Debug] OutputArea: {}x{}, Scroll Offset: {:?}, Total Lines: {}",
        frame_context.output_area.width.saturating_sub(2),
        frame_context.output_area.height.saturating_sub(2),
        scroll_offset,
        output_line_count
    ))];
    let debug_bar = Paragraph::new(debug_text);

    frame.render_widget(debug_bar, area);
}
