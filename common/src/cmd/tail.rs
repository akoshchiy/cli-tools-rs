use serde::Deserialize;

use super::{CmdExecutor, ssh::SshExecutor, CmdStream};


#[derive(Deserialize)]
pub struct TailConfig {
    pub file: String,
    pub ssh_config: String,
}

pub struct TailExecutor {
    ssh: SshExecutor,
    file: String
}

impl TailExecutor {
    pub fn new(file: String, ssh: SshExecutor) -> Self {
        Self { ssh, file }
    }
    
    fn prepare_cmd(&self, cmd: &str) -> String {
        format!("tail {} {}", cmd, self.file)
    }
}

impl CmdExecutor for TailExecutor {
    fn execute(&self, cmd: &str) -> CmdStream {
        self.ssh.execute(&self.prepare_cmd(cmd))
    }
}
