use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use reqwest::blocking::Client;

use crate::{api::Reminder, conf};

pub fn run(client: Client) {
    let token = conf::read_token().expect("Failed to read token");
    let reminder = ask().expect("Failed to ask user information about new reminder");
    reminder
        .create(&client, &token)
        .expect("Failed to create reminder");
    println!("\nCreated reminder");
}

fn ask() -> Result<Reminder> {
    let theme = ColorfulTheme::default();
    let text = Input::<String>::with_theme(&theme)
        .with_prompt("Reminder")
        .interact()?;
    let url = Input::<String>::with_theme(&theme)
        .with_prompt("URL (optional)")
        .allow_empty(true)
        .interact()?;
    Ok(Reminder {
        text,
        url: if url.is_empty() { None } else { Some(url) },
        id: None,
        created_at: None,
        age: None,
    })
}
