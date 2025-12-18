use crate::types::LoginRequest;
use clap::{Parser, Subcommand, ValueEnum};
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
    DeleteApiKey {
        #[arg(short, long)]
        key: String,
    },
    NewApiKey,
    TrackUsage {
        #[arg(short)]
        service: Service,
    },
    Balance,
    GetPaymentInfo,
    SubmitPayment {
        #[arg(short, long)]
        hash: String,
        #[arg(short, long)]
        plan: Option<String>,
    },
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Service {
    Rpc,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_input: CloudCli = CloudCli::parse();
    let mut term = Term::stdout();
    term.clear_screen()?;
    term.set_title("Developer DAO Cloud");
    term.write_line("Login to Developer DAO Cloud")?;
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
        .error_for_status()?;

    match cli_input.cmd {
        Command::GetApiKey { unsafe_print } => {
            get_keys_interactive(&client, &mut term, unsafe_print).await?
        }
        Command::DeleteApiKey { key: _ } => todo!(),
        Command::NewApiKey => todo!(),
        Command::TrackUsage { service: _ } => todo!(),
        Command::Balance => todo!(),
        Command::GetPaymentInfo => todo!(),
        Command::SubmitPayment { hash: _, plan: _ } => todo!(),
    };

    Ok(())
}
