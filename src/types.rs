use bytes::Bytes;
use cookie_store::{CookieStore, RawCookie};
use reqwest::{cookie::CookieStore as Store, header::HeaderValue};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Mutex};

pub const ASCII_ART: &str = {
    r#"
██████╗         ██████╗      ██████╗██╗      ██████╗ ██╗   ██╗██████╗ 
██╔══██╗        ██╔══██╗    ██╔════╝██║     ██╔═══██╗██║   ██║██╔══██╗
██║  ██║        ██║  ██║    ██║     ██║     ██║   ██║██║   ██║██║  ██║
██║  ██║        ██║  ██║    ██║     ██║     ██║   ██║██║   ██║██║  ██║
██████╔╝███████╗██████╔╝    ╚██████╗███████╗╚██████╔╝╚██████╔╝██████╔╝
╚═════╝ ╚══════╝╚═════╝      ╚═════╝╚══════╝ ╚═════╝  ╚═════╝ ╚═════╝ 
    "#
};

pub const ABOUT: &str = "Welcome to the D_D Cloud CLI tool. Conveniently manage API keys, track service usage, and view account balances all from your terminal.";

pub static CHAINS: &[Chains] = &[
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    pub apikey: String,
}

impl Key {
    pub fn as_redacted(&self) -> RedactedKey {
        let last_five = &self.apikey[self.apikey.len() - 5..];
        let first_five = &self.apikey[..5];
        let redacted = format!("{first_five}*****************{last_five}");
        RedactedKey { redacted }
    }
}

pub struct RedactedKey {
    pub redacted: String,
}

impl Display for RedactedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.redacted)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.apikey)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cookies(pub Mutex<CookieStore>);

impl Store for Cookies {
    fn set_cookies(
        &self,
        cookie_headers: &mut dyn Iterator<Item = &reqwest::header::HeaderValue>,
        url: &url::Url,
    ) {
        let cookies = cookie_headers
            .filter_map(|f| f.to_str().ok())
            .map(String::from)
            .filter_map(|f| RawCookie::parse(f).ok());

        self.0.lock().unwrap().store_response_cookies(cookies, url);
    }

    fn cookies(&self, url: &url::Url) -> Option<reqwest::header::HeaderValue> {
        let s = self
            .0
            .lock()
            .unwrap()
            .get_request_values(url)
            .map(|(name, value)| format!("{}={}", name, value))
            .fold(String::new(), |mut acc, e| {
                acc.push_str(&e);
                acc.push_str("; ");
                acc
            });

        if s.is_empty() {
            return None;
        }

        HeaderValue::from_maybe_shared(Bytes::from(s)).ok()
    }
}
