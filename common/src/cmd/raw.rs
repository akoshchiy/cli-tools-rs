use serde::Deserialize;

use super::{CmdExecutor, ssh::SshExecutor};

#[derive(Deserialize)]
pub struct RawConfig {
    pub cmd: String,
    pub ssh_config: String 
}

pub struct RawExecutor {
    ssh: SshExecutor,
    cmd: String,
}

impl RawExecutor {
    pub fn new(cmd: String, ssh: SshExecutor) -> Self {
        Self { cmd, ssh }
    }
}

impl CmdExecutor for RawExecutor {
    fn execute(&self, _cmd: &str) -> super::CmdStream {
        self.ssh.execute(&self.cmd)
    }
}