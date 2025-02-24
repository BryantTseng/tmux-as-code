use clap::{Arg, ArgMatches, Command};
use tracing::info;

use crate::{config::parser::AppConfig, r#const::*};

pub struct CliPrint {}

impl CliPrint {
    pub fn command() -> Command {
        Command::new(TAC_CMD_PRINT)
            .about("Print the config file")
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
        print_config_session(&config);
    }
}

fn print_config_session(config: &AppConfig) {
    for session in config.sessions.iter() {
        info!("session: {}", session.0)
    }
}
