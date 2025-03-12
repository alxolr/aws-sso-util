use serde::Deserialize;
use std::process::Command;

use crate::{aws_profiles::Profile, error::Result};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credenials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleCredentials {
    role_credentials: Credenials,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    access_token: String,
}

pub fn get_role_credentials(profile: &Profile) -> Result<Credenials> {
    let aws_sso_cache_dir = dirs::home_dir()
        .expect("Unable to get home directory")
        .join(".aws")
        .join("sso")
        .join("cache");

    // load the last file in the directory

    let last_file = aws_sso_cache_dir
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .max_by_key(|entry| entry.metadata().unwrap().modified().unwrap())
        .expect("No files found in cache");

    let storage: Storage = serde_json::from_str(&std::fs::read_to_string(last_file.path())?)?;

    let command = format!(
        "aws sso get-role-credentials --account-id {} --role-name {} --access-token {} --region {}",
        profile.sso_account_id, profile.sso_role_name, storage.access_token, profile.region
    );

    let output = Command::new("sh").arg("-c").arg(command).output()?;
    let output: RoleCredentials = serde_json::from_slice(&output.stdout)?;

    Ok(output.role_credentials)
}
