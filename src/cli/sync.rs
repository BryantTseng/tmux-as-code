use clap::{Arg, ArgMatches, Command};

use crate::{config::parser::AppConfig, r#const::*, tmux_connector};

pub struct CliSync {}

impl CliSync {
    pub fn command() -> Command {
        Command::new(TAC_CMD_SYNC)
            .alias(TAC_CMD_SYNC_ALIAS)
            .about("Sync the tmux from config")
            .arg(
                Arg::new(TAC_ARG_PATH)
                    .short(TAC_ARG_PATH_SHORT)
                    .help(TAC_ARG_PATH_HELP)
                    .default_value(TAC_DEFAULT_CONFIG_PATH_FROM_HOME)
                    .required(false),
            )
    }
    pub fn execute(arg_matches: &ArgMatches) {
        let config_path = arg_matches
            .get_one::<String>(TAC_ARG_PATH)
            .expect("Can't get ARG `path`");

        let config = AppConfig::init(config_path);

        let tmux = tmux_connector::TmuxConnector::init();

        for (name, config) in config.sessions {
            tmux.new_session(&name, config);
        }
        tmux.delete_session(None)
    }
}
