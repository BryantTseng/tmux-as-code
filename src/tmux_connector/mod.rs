use crate::{config::parser::SessionConfig, r#const::*};
use std::{process::exit, thread, time::Duration};
use tmux_interface::{
    HasSession, KillServer, KillSession, NewSession, NewWindow, StartServer, Tmux,
};
use tracing::{debug, error, info};

pub struct TmuxConnector<'a> {
    tmux: Tmux<'a>,
}

impl TmuxConnector<'_> {
    pub fn init() -> Self {
        let connector = Self { tmux: Tmux::new() };
        if connector.has_session() {
            debug!("Remove existing sessions");
            connector.kill_server();
        }
        connector.start_server_with_placeholder(TAC_SESSION_PLACEHOLDER);

        connector
    }
    pub fn start_server_with_placeholder(&self, name: &str) {
        debug!("Creating placeholder session");
        let output = self
            .tmux
            .clone()
            .add_command(StartServer::new())
            .add_command(NewSession::new().session_name(name).detached())
            .output()
            .expect("Can't create new session");
        if !output.success() {
            error!(
                "Can't create new session: {}",
                String::from_utf8(output.stderr()).unwrap()
            );
            exit(1)
        }
    }

    pub fn new_session(&self, name: &str, config: SessionConfig) {
        info!("Creating session: {}", name);
        let mut command = self
            .tmux
            .clone()
            .add_command(NewSession::new().session_name(name).detached());

        for (name, _) in config.windows {
            command = command.add_command(NewWindow::new().window_name(name));
        }
        let output = command.output().expect("Can't create new session");
        if !output.success() {
            error!(
                "Can't create new session: {}",
                String::from_utf8(output.stderr()).unwrap()
            );
            exit(1)
        }
    }
    pub fn kill_server(&self) {
        let output = self
            .tmux
            .clone()
            .add_command(KillServer::new())
            .output()
            .expect("Can't kill server");
        if !output.success() {
            error!(
                "Can't kill server: {}",
                String::from_utf8(output.stderr()).unwrap()
            );
            exit(1)
        }
        thread::sleep(Duration::from_secs(1)) // Need to wait for killserver to exit
    }
    pub fn has_session(&self) -> bool {
        let output = self
            .tmux
            .clone()
            .add_command(HasSession::new())
            .output()
            .expect("Can't check if tmux has session");
        if output.success() {
            true
        } else {
            error!(
                "Can't check if tmux has session: {}",
                String::from_utf8(output.stderr()).unwrap()
            );
            false
        }
    }
    pub fn delete_session(&self, name: Option<&str>) {
        let name = if let Some(n) = name {
            n
        } else {
            TAC_SESSION_PLACEHOLDER
        };
        let output = self
            .tmux
            .clone()
            .add_command(KillSession::new().target_session(name))
            .output()
            .expect("Can't kill session");
        if !output.success() {
            error!(
                "Can't kill session: {}",
                String::from_utf8(output.stderr()).unwrap()
            );
        }
    }
}
