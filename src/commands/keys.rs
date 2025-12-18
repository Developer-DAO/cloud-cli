use arboard::Clipboard;
use console::Term;
use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use reqwest::Client;

use crate::types::{CHAINS, Key, RedactedKey};

pub async fn get_keys_interactive(
    client: &Client,
    term: &mut Term,
    _unsafe_op: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;

    let chain = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which chain would you like to use?")
        .items(&CHAINS)
        .interact()?;

    let api_keys: Vec<Key> = client
        .get("https://api.cloud.developerdao.com/api/keys")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Key>>()
        .await?;

    term.flush()?;

    if api_keys.is_empty() {
        let new_key = Key {
            apikey: client
                .post("https://api.cloud.developerdao.com/api/keys")
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?,
        };

        let redacted_key = new_key.as_redacted();
        let redacted_ep = format!(
            "https://api.cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].id(),
            redacted_key
        );

        let endpoint = format!(
            "https://api.cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].id(),
            new_key
        );

        println!("\nYour RPC Endpoint for {}:\n", CHAINS[chain]);
        println!("\n{redacted_ep}\n",);

        match _unsafe_op {
            true => {
                term.clear_screen()?;
                println!("{endpoint}")
            }
            false => {
                clipboard.set_text(&endpoint)?;
                println!("\nYour endpoint is now copied to your clipboard");
            }
        }
    } else {
        let redacted_keys = api_keys
            .iter()
            .map(|e| e.as_redacted())
            .collect::<Vec<RedactedKey>>();

        let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
            .items(&redacted_keys)
            .with_prompt("Select an API key to copy")
            .interact()?;

        let redacted_ep = format!(
            "https://api.cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].id(),
            redacted_keys[index].redacted,
        );

        let endpoint = format!(
            "https://api.cloud.developerdao.com/rpc/{}/{}",
            CHAINS[chain].id(),
            api_keys[index].apikey
        );

        println!("\nYour RPC Endpoint for {}:\n", CHAINS[chain]);
        println!("\n{redacted_ep}\n",);

        match _unsafe_op {
            true => {
                term.clear_screen()?;
                println!("{endpoint}")
            }
            false => {
                clipboard.set_text(&endpoint)?;
                println!("\nYour endpoint is now copied to your clipboard");
                term.write_line("")?;
                term.write_line("")?;
            }
        }
    }

    Ok(())
}
