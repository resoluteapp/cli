use clap::{App, AppSettings, ArgMatches};

pub fn setup() -> ArgMatches {
    App::new("resolute")
        .about("Command line tool for useresolute.com")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Resolute Team")
        .get_matches()
}