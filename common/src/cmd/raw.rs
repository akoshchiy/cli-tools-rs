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

    fn format_cmd(&self, cmd: &str) -> String {
        format!("{} {}", self.cmd, cmd)
    }
}

impl CmdExecutor for RawExecutor {
    fn execute(&self, cmd: &str) -> super::CmdStream {
        self.ssh.execute(&self.format_cmd(cmd))
    }
}