use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, Utc};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://useresolute.com/api";

#[derive(Serialize, Deserialize, Debug)]
pub struct Reminder {
    #[serde(skip_serializing)]
    pub id: Option<u32>,
    pub text: String,
    #[serde(skip_serializing)]
    pub created_at: Option<DateTime<Utc>>,
    pub url: Option<String>,
    #[serde(skip)]
    pub age: Option<Duration>,
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

        let mut reminders: Vec<Reminder> =
            serde_json::from_str(&response.text()?).context("Failed to parse request")?;

        let now = Local::now();
        for reminder in &mut reminders {
            reminder.age = Some(reminder.created_at.unwrap().signed_duration_since(now));
        }

        Ok(reminders)
    }

    pub fn create(&self, client: &Client, token: &str) -> Result<()> {
        let response = client
            .post(format!("{}/reminders", API_URL))
            .bearer_auth(token)
            .json(self)
            .send()
            .context("Failed to send request")?;
        anyhow::ensure!(
            response.status() == StatusCode::OK,
            "Response didn't have status code of 200"
        );
        Ok(())
    }
}
