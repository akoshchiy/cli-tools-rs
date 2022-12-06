use std::collections::HashMap;

use anyhow::{anyhow, Result, Ok};
use futures::Stream;
use serde::Deserialize;
use serde_yaml::Value;

use self::{ssh::{SshExecutor, SshConfig}, tail::{TailExecutor, TailConfig}, raw::{RawExecutor, RawConfig}};

pub mod ssh;
pub mod tail;
pub mod raw;

type CmdStream = Box<dyn Stream<Item = Result<String>> + Unpin>;

pub trait CmdExecutor {
    fn execute(&self, cmd: &str) -> CmdStream;
}

#[derive(Debug, Deserialize)]
pub enum ExecutorKind {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "tail")]
    Tail,
}

#[derive(Deserialize)]
pub struct RootConfig {
    pub ssh_configs: HashMap<String, SshConfig>,
    pub executors: HashMap<String, ExecutorConfig>,
}

impl RootConfig {
    pub fn parse(val: &str) -> Result<Self> {
        let config: RootConfig = serde_yaml::from_str(val)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
pub struct ExecutorConfig {
    pub kind: ExecutorKind,
    pub config: Value,
}

pub struct CmdExecutorFactory {
    config: RootConfig,
}

impl CmdExecutorFactory {
    pub fn new(config: RootConfig) -> Self {
        Self { config }
    }

    pub async fn create(&self, id: &str) -> Result<Box<dyn CmdExecutor>> {
        let config = self.config.executors
            .get(id)
            .ok_or(anyhow!("undefined config for: {}", id))?;

        let value = config.config.clone();

        let executor: Box<dyn CmdExecutor> = match config.kind {
            ExecutorKind::Raw => Box::new(self.raw_executor(value).await?),
            ExecutorKind::Tail => Box::new(self.tail_executor(value).await?),
        };

        Ok(executor)
    }

    async fn raw_executor(&self, config: Value) -> Result<RawExecutor> {
        let raw_config: RawConfig = serde_yaml::from_value(config)?;
        let ssh = self.ssh_executor(&raw_config.ssh_config).await?;
        Ok(RawExecutor::new(
            raw_config.cmd.clone(),
            ssh
        ))
    }

    async fn ssh_executor(&self, config_id: &str) -> Result<SshExecutor> {
        let config = self.config.ssh_configs
            .get(config_id)
            .ok_or(anyhow!("undefined ssh_config for: {}", config_id))?;
        SshExecutor::new(config).await
    }

    async fn tail_executor(&self, config: Value) -> Result<TailExecutor> {
        let tail_config: TailConfig = serde_yaml::from_value(config)?;
        let ssh = self.ssh_executor(&tail_config.ssh_config).await?;
        Ok(TailExecutor::new(
            tail_config.file.clone(), 
            ssh
        ))
    }
}