pub struct ExitCode(pub i32);

impl ExitCode {
    pub fn is_success(&self) -> bool {
        self.0 == 0
    }
}
