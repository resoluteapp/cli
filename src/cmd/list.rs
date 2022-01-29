use reqwest::blocking::Client;

use crate::{api::Reminder, conf};

pub fn run(client: Client) {
    let token = conf::read_token().expect("Failed to read token");
    Reminder::fetch_all(&client, &token).expect("Failed to fetch reminders");
}
