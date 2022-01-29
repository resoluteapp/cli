use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use reqwest::blocking::Client;

use crate::{api::Reminder, conf};

pub fn run() {
    let client = Client::new();
    let token = conf::read_token().expect("Failed to read token");
    let reminders = Reminder::fetch_all(&client, &token).expect("Failed to fetch reminders");
    if reminders.is_empty() {
        println!("No reminders found");
        return;
    }
    let reminder_to_delete = reminders
        .get(ask(&reminders).expect("Failed to ask user for reminder to delete"))
        .unwrap();
    reminder_to_delete
        .delete(&client, &token)
        .expect("Failed to delete reminder");
    println!("Deleted \"{}\"", reminder_to_delete.text);
}

fn ask(reminders: &Vec<Reminder>) -> Result<usize> {
    let formatted_reminders: Vec<String> = reminders.iter().map(|r| r.format()).collect();
    Ok(FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a reminder to delete")
        .default(0)
        .items(&formatted_reminders)
        .interact()
        .context("Failed to have user select a reminder")?)
}
