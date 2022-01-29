use std::{fs, path::PathBuf};

use anyhow::Result;
use dirs::home_dir;

pub fn read_token() -> Result<String> {
    Ok(fs::read_to_string(resolute_config_dir().join("token.txt"))?
        .trim()
        .to_string())
}

fn resolute_config_dir() -> PathBuf {
    home_dir().unwrap().join(".config").join("resolute")
}
