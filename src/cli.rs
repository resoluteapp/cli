use clap::{App, AppSettings, Arg, ArgMatches};

pub fn setup() -> ArgMatches {
    App::new("resolute")
        .about("Command line tool for useresolute.com")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Resolute Team")
        .subcommand(App::new("list").about("List reminders"))
        .subcommand(
            App::new("create")
                .about("Create a reminder")
                .arg(Arg::new("reminder").index(1).required(true)),
        )
        .subcommand(
            App::new("delete")
                .about("Delete a reminder")
                .arg(Arg::new("id").index(1)),
        )
        .get_matches()
}
