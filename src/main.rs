use std::fmt::Display;

use arboard::Clipboard;
use console::Term;
use dialoguer::{FuzzySelect, Input, Password, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

static CHAINS: &'static [Chains] = &[
    Chains::Ethereum,
    Chains::Base,
    Chains::Arbitrum,
    Chains::Polygon,
    Chains::Optimism,
    Chains::BinanceSmartChain,
    Chains::Solana,
    Chains::Sui,
];

impl Display for Chains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chains::Ethereum => write!(f, "Ethereum"),
            Chains::Base => write!(f, "Base"),
            Chains::Arbitrum => write!(f, "Arbitrum"),
            Chains::Polygon => write!(f, "Polygon"),
            Chains::Optimism => write!(f, "Optimism"),
            Chains::BinanceSmartChain => write!(f, "Binance Smart Chain"),
            Chains::Solana => write!(f, "Solana"),
            Chains::Sui => write!(f, "Sui"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Chains {
    Ethereum,
    Base,
    Arbitrum,
    Polygon,
    Optimism,
    BinanceSmartChain,
    Solana,
    Sui,
}

impl Chains {
    pub fn id(&self) -> &'static str {
        match self {
            Chains::Ethereum => "eth",
            Chains::Base => "base",
            Chains::Arbitrum => "arb-one",
            Chains::Polygon => "poly",
            Chains::Optimism => "op",
            Chains::BinanceSmartChain => "bsc",
            Chains::Solana => "solana",
            Chains::Sui => "sui",
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
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

    let _ = term.clear_last_lines(4);

    let login = LoginRequest { email, password };

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let login_req = client
        .post("https://api.cloud.developerdao.com/api/login")
        .json(&login)
        .send()
        .await?;

    if login_req.status() != 200 {
        Err("Failed to authenticate user")?
    }

    term.write_line("Successfully authenticated to D_D Cloud!")?;

    let chain = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which chain would you like to use?")
        .items(&CHAINS)
        .interact()?;

    let api_keys = client
        .get("https://api.cloud.developerdao.com/api/keys")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Keys>>()
        .await?;
    let _ = term.clear_screen();
    term.flush()?;
    let mut clipboard = Clipboard::new()?;
    if api_keys.is_empty() {
        let new_key = client
            .post("https://api.cloud.developerdao.com/api/keys")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let endpoint = format!(
            "https://api.cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].id(),
            new_key
        );

        clipboard.set_text(&endpoint)?;
        println!("\nYour RPC Endpoint for {}:\n", CHAINS[chain]);
        println!("\n{endpoint}\n");
        println!("\nYour API key is now copied to your clipboard");
    } else {
        let index = dialoguer::Select::new()
            .items(&api_keys)
            .with_prompt("Select an API key to copy")
            .interact()?;

        let endpoint = format!(
            "https://api.cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].id(),
            api_keys[index].apikey
        );
        clipboard.set_text(&endpoint)?;

        println!("\nYour RPC Endpoint for {}:\n", CHAINS[chain]);
        println!("\n{endpoint}\n",);
        println!("\nYour endpoint is now copied to your clipboard");
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keys {
    apikey: String,
}

impl Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.apikey)
    }
}
