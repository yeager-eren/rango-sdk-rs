use serde::Deserialize;

use super::transaction::AssetWithTicker;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    pub block_chain: String,
    pub method: String,
    pub asset: AssetWithTicker,
    pub amount: String,
    pub decimals: u32,
    pub from_wallet_address: String,
    pub recipient_address: String,
    pub memo: Option<String>,
}
