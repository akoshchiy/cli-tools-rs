use std::env;

use clap::{command, Arg, ArgAction};
use common::run::run_executor;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(Arg::new("executor-id").required(true).action(ArgAction::Set))
        .get_matches();

    let executor_id = matches.get_one::<String>("executor-id").unwrap();
    let config_path = env::var("RCAT_CONFIG").expect("Undefined config env variable: RCAT_CONFIG");

    run_executor(executor_id, "", &config_path).await;
}