use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;

const API_URL: &str = "https://useresolute.com/api";

#[derive(Deserialize, Debug)]
pub struct Reminder {
    pub id: u32,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

impl Reminder {
    pub fn fetch_all(client: &Client, token: &str) -> Result<Vec<Self>> {
        let response = client
            .get(format!("{}/reminders", API_URL))
            .bearer_auth(token)
            .send()
            .context("Failed to send request")?;
        anyhow::ensure!(
            response.status() == StatusCode::OK,
            "Response didn't have status code of 200"
        );

        let reminders: Vec<Reminder> =
            serde_json::from_str(&response.text()?).context("Failed to parse request")?;
        Ok(reminders)
    }
}
