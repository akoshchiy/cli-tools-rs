use std::{path::Path, net::SocketAddr, io::{stdout, Write}};

use common::cmd::{ssh::{Session, SshConfig, SshExecutor}, CmdExecutor};
use futures::StreamExt;
use yaml_rust::YamlLoader;

#[tokio::main]
async fn main() {
    // YamlLoader::load_from_str(source);
    
    let config = SshConfig {
        key_path: "/Users/user/.ssh/id_rsa".to_string(),
        addr: "192.168.1.1:22".to_string(),
        user: "user".to_owned(),
        cmd: "tail -f /var/log/service".to_string() 
    };

    let executor = SshExecutor::new(config).await.unwrap();

    let mut stream = executor.execute();

    while let Some(out) = stream.next().await  {
        match out {
            Ok(data) => stdout().write_all(data.as_bytes()).unwrap(),
            Err(err) => { println!("{}", err) },
        }
    }
}


