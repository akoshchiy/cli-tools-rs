use std::{io::{stdout, Write}, env, fs::File};
use clap::{command, Arg, ArgAction, ArgMatches};
use common::cmd::{CmdExecutorFactory, RootConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(Arg::new("executor-id").required(true).action(ArgAction::Set))
        .arg(Arg::new("follow").short('f').action(ArgAction::SetTrue))
        .arg(Arg::new("number").short('n').action(ArgAction::Set))
        .get_matches();

    let executor_id = matches.get_one::<String>("executor-id").unwrap();

    let number_flag = parse_number_flag(&matches);
    let follow_flag = parse_follow_flag(&matches);
    let tail_cmd = format!("{} {}", number_flag, follow_flag);

    let config_path = env::var("RTAIL_CONFIG").expect("Undefined config env variable: RTAIL_CONFIG");
    let config_file = File::open(config_path).unwrap();
    let config: RootConfig = serde_yaml::from_reader(config_file).unwrap();

    let factory = CmdExecutorFactory::new(config);
    let executor = factory.create(executor_id).await.unwrap();

    let mut stream = executor.execute(&tail_cmd);

    while let Some(out) = stream.next().await {
        match out {
            Ok(data) => stdout().write_all(data.as_bytes()).unwrap(),
            Err(err) => { println!("{}", err) },
        }
    }
}

fn parse_number_flag(matches: &ArgMatches) -> String {
    let number_cmd = matches.get_one::<String>("number");
    match number_cmd {
        Some(n) => format!("-n {}", n),
        None => "".to_string(),
    }
}

fn parse_follow_flag(matches: &ArgMatches) -> String {
    let follow = matches.get_flag("follow");
    if follow { "-f".to_owned() } else { "".to_owned() }
}