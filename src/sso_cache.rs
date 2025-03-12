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
    start_url: String,
}

pub fn get_load_storage() -> Result<Storage> {
    let aws_sso_cache_dir = dirs::home_dir()
        .expect("Unable to get home directory")
        .join(".aws")
        .join("sso")
        .join("cache");

    let last_file = std::fs::read_dir(aws_sso_cache_dir)?
        .flat_map(|entry| entry.map(|e| e.path()))
        .filter(|path| path.is_file())
        .max_by_key(|path| {
            path.metadata()
                .expect("Unable to get metadata")
                .modified()
                .expect("Unable to get modified time")
        })
        .expect("No cache file found");

    let storage: Storage = serde_json::from_str(&std::fs::read_to_string(last_file)?)?;

    Ok(storage)
}

pub fn get_role_credentials(profile: &Profile) -> Result<Credenials> {
    let storage = get_load_storage()?;
    let command = format!(
        "aws sso get-role-credentials --account-id {} --role-name {} --access-token {} --region {}",
        profile.sso_account_id, profile.sso_role_name, storage.access_token, profile.region
    );

    let output = Command::new("sh").arg("-c").arg(command).output()?;
    let output: RoleCredentials = serde_json::from_slice(&output.stdout)?;

    Ok(output.role_credentials)
}

pub fn get_console_url(profile: &Profile) -> Result<String> {
    let storage = get_load_storage()?;
    Ok(format!(
        "{}console?account_id={}&role_name={}&region={}",
        storage.start_url, profile.sso_account_id, profile.sso_role_name, profile.region
    ))
}
