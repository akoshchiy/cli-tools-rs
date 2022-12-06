use std::{io::{stdout, Write}, env, fs::File};
use clap::{command, Arg, ArgAction};
use common::cmd::{CmdExecutorFactory, RootConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(Arg::new("executor-id").required(true).action(ArgAction::Set))
        .arg(Arg::new("command").required(false).action(ArgAction::Set))
        .get_matches();

    let executor_id = matches.get_one::<String>("executor-id").unwrap();

    let default_cmd: String = "".to_string();
    let command = matches.get_one::<String>("command").unwrap_or(&default_cmd);

    let config_path = env::var("RTAIL_CONFIG").unwrap();
    let config_file = File::open(config_path).unwrap();
    let config: RootConfig = serde_yaml::from_reader(config_file).unwrap();

    let factory = CmdExecutorFactory::new(config);
    let executor = factory.create(executor_id).await.unwrap();

    let mut stream = executor.execute(command);

    while let Some(out) = stream.next().await {
        match out {
            Ok(data) => stdout().write_all(data.as_bytes()).unwrap(),
            Err(err) => { println!("{}", err) },
        }
    }
}