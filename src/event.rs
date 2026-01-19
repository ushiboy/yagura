use crossterm::event::KeyEvent;
use uuid::Uuid;

use crate::app::OutputLine;

pub enum AppEvent {
    Key(KeyEvent),
    ProcessOutput(Uuid, OutputLine),
}
