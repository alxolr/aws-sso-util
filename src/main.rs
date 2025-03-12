use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use error::Result;
use structopt::StructOpt;

mod aws_profiles;
mod error;
mod sso_cache;

#[derive(structopt::StructOpt, Debug)]
#[structopt(
    name = "aws-sso-util",
    bin_name = "aws-sso-util",
    about = "AWS SSO utility"
)]
pub struct Opt {
    #[structopt(short, long, help = "Fuzzy search for aws profiles")]
    profile: bool,

    #[structopt(short, long, help = "Get aws console url for the selected profile")]
    console_ui: bool,

    #[structopt(
        short,
        long,
        help = "Get the export env for aws credentials for the selected profile"
    )]
    env: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    if !opt.console_ui && !opt.env && !opt.profile {
        println!("No options selected");
        println!("Run with --help for more information");
        return Ok(());
    }

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

    if opt.console_ui {
        let url = sso_cache::get_console_url(profile)?;
        println!("{}", url);
    }

    if opt.env {
        let creadentials = sso_cache::get_role_credentials(profile)?;
        println!("export AWS_ACCESS_KEY_ID={}", creadentials.access_key_id);
        println!(
            "export AWS_SECRET_ACCESS_KEY={}",
            creadentials.secret_access_key
        );
        println!("export AWS_SESSION_TOKEN={}", creadentials.session_token);
    }

    Ok(())
}
