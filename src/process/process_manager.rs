use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Child;
use tokio::sync::Mutex;

use crate::process::Pid;

use uuid::Uuid;

pub struct ProcessManager {
    handlers: HashMap<Uuid, ProcessHandle>,
}

pub struct ProcessHandle {
    _pid: Pid,
    child: Arc<Mutex<Child>>,
}

mod init;
mod shutdown_all;
mod spawn;
mod stop;
