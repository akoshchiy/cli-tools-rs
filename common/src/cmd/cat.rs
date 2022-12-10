use serde::Deserialize;

use super::{ssh::SshExecutor, CmdExecutor, CmdStream};

#[derive(Deserialize)]
pub struct CatConfig {
    pub file: String,
    pub ssh_config: String,
}

pub struct CatExecutor {
    ssh: SshExecutor,
    file: String
}

impl CatExecutor {
    pub fn new(file: String, ssh: SshExecutor) -> Self {
        Self { ssh, file }
    }
    
    fn prepare_cmd(&self) -> String {
        format!("cat {}", self.file)
    }
}

impl CmdExecutor for CatExecutor {
    fn execute(&self, _cmd: &str) -> CmdStream {
        self.ssh.execute(&self.prepare_cmd())
    }
}
