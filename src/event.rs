use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::{app::OutputLine, process::ExitCode};

pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    ProcessOutput(Uuid, OutputLine),
    ProcessExited(Uuid, ExitCode),
}
