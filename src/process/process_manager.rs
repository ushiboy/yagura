use std::collections::HashMap;
use tokio::sync::oneshot;

use crate::process::ProcessId;

use uuid::Uuid;

pub struct ProcessManager {
    handlers: HashMap<Uuid, ProcessHandle>,
}

pub struct ProcessHandle {
    _pid: ProcessId,
    kill_tx: Option<oneshot::Sender<()>>,
}

mod init;
mod on_process_existed;
mod shutdown_all;
mod spawn;
mod stop;
