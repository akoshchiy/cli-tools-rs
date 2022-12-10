use std::env;
use clap::{command, Arg, ArgAction, ArgMatches};
use common::run::run_executor;

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

    run_executor(executor_id, &tail_cmd, &config_path).await;
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