mod commands;
mod context;

use std::collections::BTreeMap;
use std::io::Write;

use clap::Command as ClapCommand;
use commands::{get_commands, Command};
use context::Context;

pub struct Cli {
    context: Context,
    command: ClapCommand,
    sub: BTreeMap<&'static str, Box<dyn Command>>,
}

impl Default for Cli {
    fn default() -> Self {
        let mut sub = BTreeMap::new();
        let mut command = ClapCommand::new("repl")
            .multicall(true)
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand_value_name("Command")
            .subcommand_help_heading("help heading");
        for item in get_commands() {
            command = command.subcommand(item.to_clap_command());
            sub.insert(item.name(), item);
        }
        Cli {
            command,
            sub,
            context: Default::default(),
        }
    }
}

impl Cli {
    fn try_match_args(&mut self, line: &str) {
        let matches = match self.command.clone().try_get_matches_from(line.split(" ")) {
            Ok(m) => m,
            Err(err) => {
                println!("{}", err.to_string());
                return;
            }
        };
        if let Some((name, matches)) = matches.subcommand() {
            if let Some(sub_command) = self.sub.get(name) {
                sub_command.run(&mut self.context, matches);
            }
        }
    }

    pub fn listen(&mut self) {
        loop {
            std::io::stdout().flush().unwrap();
            write!(std::io::stdout(), "$ ").unwrap();
            std::io::stdout().flush().unwrap();
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let line = buffer.trim();
            if line.is_empty() {
                continue;
            }

            self.try_match_args(line);
        }
    }
}
