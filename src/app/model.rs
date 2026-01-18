pub struct App {
    should_quit: bool,
}

impl App {
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

mod init;
mod quit;
