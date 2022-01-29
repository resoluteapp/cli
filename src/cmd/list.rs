use chrono_humanize::HumanTime;
use reqwest::blocking::Client;

use crate::{api::Reminder, conf};

pub fn run(client: Client) {
    let token = conf::read_token().expect("Failed to read token");
    let reminders = Reminder::fetch_all(&client, &token).expect("Failed to fetch reminders");
    output(reminders);
}

fn output(reminders: Vec<Reminder>) {
    for reminder in reminders {
        println!(
            "- {} ({})",
            reminder.text,
            HumanTime::from(reminder.age.unwrap())
        )
    }
}
