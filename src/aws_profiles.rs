use std::{collections::HashMap, fs};

use ini::configparser::ini::Ini;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub sso_session: String,
    pub sso_account_id: String,
    pub sso_role_name: String,
    pub region: String,
}

pub fn get_profiles() -> Result<Vec<Profile>> {
    let aws_config_path = dirs::home_dir()
        .expect("Unable to get home directory")
        .join(".aws")
        .join("config");

    let aws_config_str = fs::read_to_string(aws_config_path)?;
    let config = Ini::new().read(aws_config_str)?;

    let profiles = config
        .iter()
        .filter(|(profile, _)| profile.starts_with("profile"))
        .map(|(profile, value)| Profile {
            name: profile.replace("profile ", ""),
            sso_session: get_value(value, "sso_session"),
            sso_account_id: get_value(value, "sso_account_id"),
            sso_role_name: get_value(value, "sso_role_name"),
            region: get_value(value, "region"),
        })
        .collect();

    Ok(profiles)
}

fn get_value(config: &HashMap<String, Option<String>>, key: &str) -> String {
    config.get(key).unwrap().as_ref().unwrap().to_string()
}
