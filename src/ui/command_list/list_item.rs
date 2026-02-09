use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::{
    model::Command,
    ui::command_list::{
        display_status::display_status, format_elapsed_time::format_elapsed_time,
        format_pid::format_pid, truncate_string::truncate_string,
    },
};

pub fn list_item(cmd: &Command, is_selected: bool) -> ListItem<'_> {
    let marker = if is_selected { "[*]" } else { "[ ]" };
    let status = display_status(cmd.status());
    let pid = format_pid(cmd.pid());
    let time = format_elapsed_time(cmd.elapsed_time());
    let content = format!(
        "{} {:<30} {} PID:{} {}",
        marker,
        truncate_string(cmd.command(), 30),
        status,
        pid,
        time
    );

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    ListItem::new(Line::from(Span::styled(content, style)))
}
