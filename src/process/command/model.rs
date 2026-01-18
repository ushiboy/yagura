use uuid::Uuid;

pub struct Command {
    id: Uuid,
    command: String,
}

impl Command {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }
}

mod init;
