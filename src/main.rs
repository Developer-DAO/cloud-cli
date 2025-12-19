use crate::{
    commands::{
        account_info::{get_balance, get_rpc_calls},
        keys::{delete_api_key, new_api_key},
    },
    types::{ASCII_ART, LoginRequest},
};
use clap::{Parser, Subcommand};
use commands::keys::get_keys_interactive;
use console::Term;
use dialoguer::{Input, Password, theme::ColorfulTheme};

pub mod commands;
pub mod types;

#[derive(Parser)]
pub struct CloudCli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    GetApiKey {
        #[arg(short, long)]
        unsafe_print: bool,
    },
    DeleteApiKey,
    NewApiKey,
    TrackUsage,
    Balance,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_input: CloudCli = CloudCli::parse();
    let mut term = Term::stdout();
    term.clear_screen()?;
    term.set_title("Developer DAO Cloud");
    term.write_line(ASCII_ART)?;
    term.write_line("Login")?;
    term.write_line("")?;

    let email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Email")
        .interact_text()?;

    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()?;

    term.clear_screen()?;

    let login = LoginRequest { email, password };

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    client
        .post("https://api.cloud.developerdao.com/api/login")
        .json(&login)
        .send()
        .await?
        .error_for_status()
        .map_err(|_| "Invalid username or password")?;

    match cli_input.cmd {
        Command::GetApiKey { unsafe_print } => {
            get_keys_interactive(&client, &mut term, unsafe_print).await?
        }
        Command::DeleteApiKey => delete_api_key(&client).await?,
        Command::NewApiKey => new_api_key(&client).await?,
        Command::TrackUsage => get_rpc_calls(&client).await?,
        Command::Balance => get_balance(&client).await?,
    };

    Ok(())
}
