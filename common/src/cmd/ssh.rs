use std::{path::Path, net::SocketAddr, time::Duration, sync::{Arc}, io::Write, iter};

use futures::{Stream, channel::mpsc::unbounded, stream, lock::Mutex, StreamExt};
use russh::{client, Disconnect};
use russh_keys::{key::PublicKey, load_secret_key};
use anyhow::{Result};


use super::{CmdExecutor, CmdStream};

pub struct SshConfig {
    pub key_path: String,
    pub user: String,
    pub addr: String,
    pub cmd: String,
}

pub struct SshExecutor {
    session: Arc<Mutex<Session>>,
    cmd: String,
}

impl SshExecutor {

    pub async fn new(config: SshConfig) -> Result<Self> {
        let user = config.user;
        let key_path = Path::new(&config.key_path);
        let addr: SocketAddr = config.addr.parse()?;
        let session = Session::connect(key_path, user, addr).await?;

        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            cmd: config.cmd,
        })
    }
}

impl CmdExecutor for SshExecutor {
    fn execute(&self) -> CmdStream {
        let (sender, recv) = unbounded();

        let session_mutex = self.session.clone();
        let cmd = self.cmd.clone();

        tokio::spawn(async move {
            let mut session = session_mutex.lock().await;
            let stream_res = session.call(&cmd).await;
            drop(session);

            if stream_res.is_err() {
                let err = stream_res.err().unwrap();
                sender.unbounded_send(Err(err)).unwrap();
            } else {
                let mut stream = stream_res.unwrap();
                while let Some(resp) = stream.next().await {
                    sender.unbounded_send(Ok(resp)).unwrap();
                }
            }
        });

         Box::new(recv)
    }
}

struct Client {}

impl client::Handler for Client {
    type Error = russh::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), Self::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), Self::Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }
    fn check_server_key(self, _server_public_key: &PublicKey) -> Self::FutureBool {
        self.finished_bool(true)
    }
}

pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    pub async fn connect<P: AsRef<Path>>(
        key_path: P,
        user: impl Into<String>,
        addr: SocketAddr,
    ) -> Result<Self> {
        let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            connection_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let sh = Client {};
        let mut session = client::connect(config, addr, sh).await?;
        let _auth_res = session
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;

        Ok(Self { session })
    }

    pub async fn call(&mut self, command: &str) -> Result<impl Stream<Item = String>> {
        let (sender, recv) = unbounded();

        let mut channel = self.session.channel_open_session().await?;

        channel.exec(true, command).await?;

        tokio::spawn(async move {
            while let Some(msg) = channel.wait().await {
                match msg {
                    russh::ChannelMsg::Data { ref data } => {
                        let resp: String = String::from_utf8_lossy(&data).into();
                        sender.unbounded_send(resp).unwrap();
                    }
                    _ => {}
                }
            }
        });

        Ok(recv)
    }
}

pub struct CommandResult {
    output: Vec<u8>,
    code: Option<u32>,
}

impl CommandResult {
    pub fn output(&self) -> String {
        String::from_utf8_lossy(&self.output).into()
    }

    pub fn success(&self) -> bool {
        self.code == Some(0)
    }
}
