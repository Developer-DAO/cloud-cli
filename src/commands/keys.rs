use arboard::Clipboard;
use aws_sdk_secretsmanager::operation::create_secret::CreateSecretOutput;
use console::Term;
use dialoguer::{Confirm, FuzzySelect, theme::ColorfulTheme};
use reqwest::Client;

use crate::{
    SecretManager,
    types::{CHAINS, Key, RedactedKey},
};

pub async fn get_keys_interactive(
    client: &Client,
    term: &mut Term,
    _unsafe_op: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;

    let chain = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which chain would you like to use?")
        .items(CHAINS)
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

pub async fn new_api_key(
    client: &Client,
    secret_manager: Option<SecretManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
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

    match secret_manager {
        Some(SecretManager::Aws) => {
            let secret_info = store_in_aws_secret_manager(new_key).await?;
            println!(
                "Successfully created your new API key:\n{redacted_key}\n Successfully added to AWS Secret Manager!\nName: {}\nARN: {}",
                secret_info.name().unwrap_or_else(|| "None"),
                secret_info.arn().unwrap_or_else(|| "None")
            );
        }
        None => {
            clipboard.set_text(new_key.apikey)?;
            println!(
                "Your new API key:\n{redacted_key}\nSuccessfully created and copied to clipboard!"
            );
        }
    }

    Ok(())
}

pub async fn delete_api_key(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let api_keys: Vec<Key> = client
        .get("https://api.cloud.developerdao.com/api/keys")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<Key>>()
        .await?;

    if api_keys.is_empty() {
        Err("Err: deletion failed. No API keys found.")?
    }

    let redacted_keys = api_keys
        .iter()
        .map(|e| e.as_redacted())
        .collect::<Vec<RedactedKey>>();

    let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .items(&redacted_keys)
        .with_prompt("Select an API key to delete")
        .interact()?;

    let confirmation = Confirm::new()
        .with_prompt(format!(
            "Are you want you want to delete {}?",
            redacted_keys[index].redacted
        ))
        .interact()
        .unwrap();

    if !confirmation {
        Err("API key deletion aborted")?
    }

    client
        .delete(format!(
            "https://api.cloud.developerdao.com/api/keys/{}",
            api_keys[index].apikey
        ))
        .send()
        .await?
        .error_for_status()?;

    println!(
        "Successfully deleted api key: {}",
        redacted_keys[index].redacted
    );

    Ok(())
}

pub async fn store_in_aws_secret_manager(
    api_key: Key,
) -> Result<CreateSecretOutput, Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_secretsmanager::Client::new(&config);
    Ok(client
        .create_secret()
        .set_name(Some(format!("D_D-Cloud-API-Key-{}", &api_key.apikey[..5])))
        .set_secret_string(Some(api_key.apikey))
        .set_description(Some("An api key for D_D Cloud made via CLI".to_string()))
        .send()
        .await?)
}
