use std::{collections::HashMap, fs};

use ini::configparser::ini::Ini;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

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
        .flat_map(|(profile, value)| {
            let profile = Profile {
                name: profile.replace("profile ", ""),
                sso_session: get_value(value, "sso_session")?,
                sso_account_id: get_value(value, "sso_account_id")?,
                sso_role_name: get_value(value, "sso_role_name")?,
                region: get_value(value, "region")?,
            };

            Ok::<Profile, Error>(profile)
        })
        .collect::<Vec<Profile>>();

    Ok(profiles)
}

fn get_value(config: &HashMap<String, Option<String>>, key: &str) -> Result<String> {
    let value = config
        .get(key)
        .ok_or(Error::KeyNotFound)?
        .as_ref()
        .ok_or(Error::ValueNotFound)?
        .to_string();

    Ok(value)
}
