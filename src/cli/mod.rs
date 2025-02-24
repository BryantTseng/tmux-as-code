pub mod print;

pub mod sync;

use clap::{Command, crate_name, crate_version};

use crate::r#const::{TAC_CMD_PRINT, TAC_CMD_SYNC};

use print::CliPrint;
use sync::CliSync;

pub struct Cli {
    command: Command,
}

impl Cli {
    pub fn new() -> Self {
        let command = Command::new(crate_name!())
            .version(crate_version!())
            .subcommand(CliSync::command())
            .subcommand(CliPrint::command())
            .about("Create tmux from config file");

        Self { command }
    }
    pub fn execute(self) {
        let matches = self.command.get_matches();

        if let Some((subcommand, arg_matches)) = matches.subcommand() {
            match subcommand {
                TAC_CMD_SYNC => {
                    CliSync::execute(arg_matches);
                }
                TAC_CMD_PRINT => {
                    CliPrint::execute(arg_matches);
                }
                _ => {}
            }
        }
    }
}
