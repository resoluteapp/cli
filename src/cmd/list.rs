use reqwest::blocking::Client;

use crate::{api::Reminder, conf};

pub fn run() {
    let client = Client::new();
    let token = conf::read_token().expect("Failed to read token");
    let reminders = Reminder::fetch_all(&client, &token).expect("Failed to fetch reminders");
    output(reminders);
}

fn output(reminders: Vec<Reminder>) {
    if reminders.is_empty() {
        println!("No reminders found");
        return;
    }
    for reminder in reminders {
        println!("- {}", reminder.format())
    }
}
