use clap::ArgMatches;
use reqwest::blocking::Client;

use crate::{api::Reminder, conf};

pub fn run(args: &ArgMatches, client: Client) {
    let token = conf::read_token().expect("Failed to read token");
    let reminder = Reminder {
        text: String::from(args.value_of("reminder").unwrap()),
        ..Reminder::default()
    };
    reminder
        .create(&client, &token)
        .expect("Failed to create reminder");
    println!("Created reminder: \"{}\"", reminder.text);
}
