use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserBalances {
    pub calls: i64,
    pub balance: i64,
}

pub async fn get_rpc_calls(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let res = client
        .get("https://api.cloud.developerdao.com/api/balances")
        .send()
        .await?
        .error_for_status()?
        .json::<UserBalances>()
        .await?;

    println!("You've made {} RPC calls this month\n", res.calls);
    Ok(())
}

pub async fn get_balance(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let res = client
        .get("https://api.cloud.developerdao.com/api/balances")
        .send()
        .await?
        .error_for_status()?
        .json::<UserBalances>()
        .await?;

    println!(
        "Your account has a balance of ${}\n",
        res.balance as f64 / 100.0
    );
    Ok(())
}
