use serde::{Deserialize, Serialize};

use super::{cosmos::CosmosTransaction, evm::EvmTransaction, transfer::TransferTransaction};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetWithTicker {
    pub blockchain: String,
    pub address: Option<String>,
    pub symbol: String,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Transaction {
    Evm(EvmTransaction),
    Cosmos(CosmosTransaction),
    Transfer(TransferTransaction),
}
