use std::{fs::File, io::{stdout, Write}};

use futures::StreamExt;

use crate::cmd::{RootConfig, CmdExecutorFactory};

pub async fn run_executor(id: &str, cmd: &str, config_path: &str) {
    let config_file = File::open(config_path).unwrap();
    let config: RootConfig = serde_yaml::from_reader(config_file).unwrap();

    let factory = CmdExecutorFactory::new(config);
    let executor = factory.create(id).await.unwrap();

    let mut stream = executor.execute(&cmd);

    while let Some(out) = stream.next().await {
        match out {
            Ok(data) => stdout().write_all(data.as_bytes()).unwrap(),
            Err(err) => { println!("{}", err) },
        }
    }
}