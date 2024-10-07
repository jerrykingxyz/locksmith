use super::{ArgMatches, ClapCommand, Command, Context};

pub struct Ls;

impl Command for Ls {
    fn name(&self) -> &'static str {
        "ls"
    }
    fn to_clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name()).about("List keys")
    }
    fn run(&self, _context: &mut Context, _args: &ArgMatches) {
        println!("ls run")
    }
}
