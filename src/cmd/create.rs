use anyhow::Result;
use chrono::Utc;
use dialoguer::{theme::ColorfulTheme, Input};
use reqwest::blocking::Client;

use crate::api::Reminder;

pub fn run(client: Client) {
    let reminder = ask().expect("Failed to ask user information about new reminder");
    println!("{:?}", reminder);
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
        id: None,
        text,
        created_at: Utc::now(),
        url: if "" == &url { None } else { Some(url) },
        age: None,
    })
}
