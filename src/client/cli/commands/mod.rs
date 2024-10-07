mod exit;
mod ls;

use super::Context;
use clap::{ArgMatches, Command as ClapCommand};

pub trait Command {
    fn name(&self) -> &'static str;
    fn to_clap_command(&self) -> ClapCommand;
    fn run(&self, context: &mut Context, args: &ArgMatches);
}

pub fn get_commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(ls::Ls), Box::new(exit::Exit)]
}
