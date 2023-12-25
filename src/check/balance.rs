use serde::{Deserialize, Serialize};

use crate::{error::SdkErr, quote::Asset};

#[derive(Debug, Serialize)]
pub struct BalanceRequest {
    pub blockchain: String,
    pub address: String,
}

impl BalanceRequest {
    pub fn into_qs(&self) -> Result<String, SdkErr> {
        let qs = serde_urlencoded::to_string(self)?;

        Ok(qs)
    }
}

#[derive(Debug, Deserialize)]
pub struct Amount {
    pub amount: String,
    pub decimals: u32,
}

#[derive(Debug, Deserialize)]
pub struct AssetAndAmount {
    pub amount: Amount,
    pub asset: Asset,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDetail {
    pub failed: bool,
    pub block_chain: String,
    pub address: String,
    pub balances: Option<Vec<AssetAndAmount>>,
    pub explorer_url: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub wallets: Vec<WalletDetail>,
}
