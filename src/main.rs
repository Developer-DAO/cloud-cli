use console::Term;
use dialoguer::{FuzzySelect, Input, Password, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};

static CHAINS: &'static [&'static str] = &[
    "ArbitrumOne",
    "ArbitrumSepoliaTestnet",
    "Avalanche",
    "AvalancheDFK",
    "Base",
    "BaseSepoliaTestnet",
    "Bitcoin",
    "Blast",
    "BNBChain",
    "Boba",
    "CelestiaConsensus",
    "CelestiaConsensusTestnet",
    "CelestiaDA",
    "CelestiaDATestnet",
    "Celo",
    "Ethereum",
    "EthereumHoleskyTestnet",
    "EthereumSepoliaTestnet",
    "Evmos",
    "Fantom",
    "Fraxtal",
    "Fuse",
    "Gnosis",
    "Harmony0",
    "IoTeX",
    "Kaia",
    "Kava",
    "Metis",
    "Moonbeam",
    "Moonriver",
    "Near",
    "OasysMainnet",
    "OpBNB",
    "Optimism",
    "OptimismSepolia",
    "Osmosis",
    "PocketNetwork",
    "Polygon",
    "PolygonAmoyTestnet",
    "PolygonzkEVM",
    "Radix",
    "Scroll",
    "Solana",
    "Sui",
    "Taiko",
    "TaikoHeklaTestnet",
    "ZkLink",
    "ZkSyncEra",
];

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
        .post("http://localhost:3000/api/login")
        .json(&login)
        .send()
        .await?;

    if login_req.status() != 200 {
        Err("Failed to authenticate user.")?
    }

    term.write_line("Successfully authenticated to D_D Cloud!")?;

    let chain = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which chain would you like to use?")
        .items(&CHAINS)
        .interact()?;

    let api_keys = client
        .get("http://localhost:3000/api/keys")
        .send()
        .await?
        .json::<Vec<Keys>>()
        .await?;
    let _ = term.clear_screen();
    term.flush()?;
    if api_keys.is_empty() {
        let new_key = client
            .post("http://localhost:3000/api/keys")
            .send()
            .await?
            .text()
            .await?;
        println!("Your RPC Endpoint for {}:\n", CHAINS[chain]);
        println!(
            "https://cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].to_lowercase(),
            new_key
        );
    } else {
        println!("Your RPC Endpoint for {}:\n", CHAINS[chain]);
        println!(
            "https://cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].to_lowercase(),
            api_keys[0].apikey
        );
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
