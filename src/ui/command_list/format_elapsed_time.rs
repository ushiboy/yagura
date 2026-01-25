use std::time::Duration;

pub fn format_elapsed_time(elapsed_time: Option<Duration>) -> String {
    if let Some(duration) = elapsed_time {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        "--:--:--".to_string()
    }
}
