use crate::{
    commands::{
        account_info::{get_balance, get_rpc_calls},
        keys::{delete_api_key, new_api_key},
    },
    types::{ABOUT, ASCII_ART, Cookies, LoginRequest},
};
use clap::{Parser, Subcommand, ValueEnum};
use commands::keys::get_keys_interactive;
use console::Term;
use cookie_store::serde::json::{load, save_incl_expired_and_nonpersistent};
use dialoguer::{Input, Password, theme::ColorfulTheme};
use std::{
    env,
    sync::{Arc, Mutex},
};

pub mod commands;
pub mod types;
#[derive(Parser)]
#[command(version, about = ABOUT, long_about = None, name = "dd-cloud", author)]
pub struct CloudCli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Retrieve one of your API key for D_D Cloud.
    GetApiKey {
        /// Flag to print the full URL including your API key to the terminal.
        /// Unsafe to use because your API key is written to stdout.
        #[arg(long)]
        unsafe_print: bool,
    },
    /// Delete one of your API keys for D_D Cloud.
    DeleteApiKey,
    /// Generate a new API key for D_D Cloud. Max 10.
    NewApiKey {
        /// Store your API key in a secret manager. Currently supports AWS.
        #[arg(short, long)]
        secret_manager: Option<SecretManager>,
    },
    /// Returns the number of calls you made this cycle.
    TrackUsage,
    /// Returns your account balance.
    Balance,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum SecretManager {
    /// Ensure to login to AWS first via CLI with `aws login`
    Aws,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_input: CloudCli = CloudCli::parse();
    let mut term = Term::stdout();
    term.clear_screen()?;
    term.set_title("Developer DAO Cloud");
    term.write_line(ASCII_ART)?;

    let mut path = env::home_dir().ok_or("Failed to retrieve a HOME path for user.")?;
    path.push(".futex-labs");

    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    path.push("credentials");

    let cookies = Arc::new(Cookies(Mutex::new(
        std::fs::File::open(&path)
            .map(std::io::BufReader::new)
            .map(|f| load(f).unwrap_or_default())
            .unwrap_or_default(),
    )));

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::clone(&cookies))
        .build()?;

    if client
        .post("https://api.cloud.developerdao.com/api/refresh")
        .send()
        .await?
        .error_for_status()
        .is_err()
    {
        term.write_line("Login")?;
        term.write_line("")?;

        let email: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Email")
            .interact_text()?;

        let password: String = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Password")
            .interact()?;

        term.clear_screen()?;
        term.write_line(ASCII_ART)?;
        term.write_line("")?;

        let login = LoginRequest { email, password };

        client
            .post("https://api.cloud.developerdao.com/api/login")
            .json(&login)
            .send()
            .await?
            .error_for_status()
            .map_err(|_| "Invalid username or password")?;
    }

    match cli_input.cmd {
        Command::GetApiKey { unsafe_print } => {
            get_keys_interactive(&client, &mut term, unsafe_print).await?
        }
        Command::DeleteApiKey => delete_api_key(&client).await?,
        Command::NewApiKey { secret_manager } => new_api_key(&client, secret_manager).await?,
        Command::TrackUsage => get_rpc_calls(&client).await?,
        Command::Balance => get_balance(&client).await?,
    };

    let mut writer = std::fs::File::create(path).map(std::io::BufWriter::new)?;
    let jar = cookies.0.lock().unwrap();
    save_incl_expired_and_nonpersistent(&jar, &mut writer)
        .map_err(|_| "Failed to write temp credentials to disk.")?;

    Ok(())
}
