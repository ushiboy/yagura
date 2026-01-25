use std::collections::HashMap;
use tokio::sync::oneshot;

use crate::process::Pid;

use uuid::Uuid;

pub struct ProcessManager {
    handlers: HashMap<Uuid, ProcessHandle>,
}

pub struct ProcessHandle {
    _pid: Pid,
    kill_tx: oneshot::Sender<()>,
}

mod init;
mod shutdown_all;
mod spawn;
mod stop;
