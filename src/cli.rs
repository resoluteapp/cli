use clap::{Arg, Command};
use clap_complete::Shell;

pub fn setup() -> Command<'static> {
    Command::new("resolute")
        .about("Command line tool for useresolute.com")
        .version("0.1.0")
        .subcommand_required(true)
        .author("Resolute Team")
        .subcommand(Command::new("list").about("List reminders"))
        .subcommand(
            Command::new("create")
                .about("Create a reminder")
                .arg(Arg::new("reminder").index(1).required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a reminder")
                .arg(Arg::new("id").index(1)),
        )
        .subcommand(
            Command::new("completion")
                .about("Generate shell completion for resolute")
                .arg(Arg::new("shell").possible_values(Shell::possible_values())),
        )
}
