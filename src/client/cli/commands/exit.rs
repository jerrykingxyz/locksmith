use super::{ArgMatches, ClapCommand, Command, Context};

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }
    fn to_clap_command(&self) -> ClapCommand {
        ClapCommand::new(self.name()).about("Quit the program")
    }
    fn run(&self, _context: &mut Context, _args: &ArgMatches) {
        println!("exit run")
    }
}
