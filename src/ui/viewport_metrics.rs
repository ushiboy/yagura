use crate::ui::FrameContext;

#[derive(Debug, Clone, Copy, Default)]
pub struct ViewportMetrics {
    pub command_list_height: usize,
    pub output_area_height: usize,
    pub help_bar_height: usize,
}

impl From<&FrameContext> for ViewportMetrics {
    fn from(frame_context: &FrameContext) -> Self {
        ViewportMetrics {
            // Subtracting 2 for borders
            command_list_height: frame_context.command_list_area.height.saturating_sub(2) as usize,
            // Subtracting 2 for borders
            output_area_height: frame_context.output_area.height.saturating_sub(2) as usize,
            help_bar_height: frame_context.help_bar_area.height as usize,
        }
    }
}
