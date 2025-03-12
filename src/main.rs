use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use error::Result;
use structopt::StructOpt;

mod aws_profiles;
mod error;
mod role_credentials;

#[derive(structopt::StructOpt)]
pub struct Opt {}

fn main() -> Result<()> {
    let _ = Opt::from_args();
    let profiles = aws_profiles::get_profiles()?;

    let options = profiles
        .iter()
        .map(|profile| profile.name.as_str())
        .collect::<Vec<&str>>();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profile:")
        .items(&options)
        .interact_opt()?;

    let profile = profiles
        .iter()
        .find(|profile| profile.name == options[selection.unwrap()])
        .expect("Better select a profile");

    let creadentials = role_credentials::get_role_credentials(profile)?;

    println!("export AWS_ACCESS_KEY_ID={}", creadentials.access_key_id);
    println!(
        "export AWS_SECRET_ACCESS_KEY={}",
        creadentials.secret_access_key
    );
    println!("export AWS_SESSION_TOKEN={}", creadentials.session_token);

    Ok(())
}
