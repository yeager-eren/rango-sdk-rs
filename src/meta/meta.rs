use super::{blockchains::BlockchainMeta, swappers::SwapperMeta};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub blockchain: String,
    pub chain_id: Option<String>,
    pub address: Option<String>,
    pub symbol: String,
    pub name: Option<String>,
    pub decimals: u32,
    pub image: String,
    pub blockchain_image: String,
    pub usd_price: Option<f64>,
    pub is_popular: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub blockchains: Vec<BlockchainMeta>,
    pub swappers: Vec<SwapperMeta>,
    pub tokens: Vec<Token>,
}
