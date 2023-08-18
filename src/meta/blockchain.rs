use serde::Deserialize;

use crate::chains::ChainInfo;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub blockchain: String,
    pub address: Option<String>,
    pub symbol: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EvmBlockchainMeta {
    // Common
    pub name: String,
    pub short_name: String,
    pub display_name: String,
    pub default_decimals: u8,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    pub logo: String,
    pub color: String,
    pub sort: u32,
    pub enabled: bool,

    // Specific
    pub chain_id: String,
    pub info: ChainInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBlockchainMeta {
    // Common
    pub name: String,
    pub short_name: String,
    pub display_name: String,
    pub default_decimals: u8,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    pub logo: String,
    pub color: String,
    pub sort: u32,
    pub enabled: bool,

    // Specific
    pub chain_id: Option<String>,
    pub info: Option<ChainInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TronBlockchainMeta {
    // Common
    pub name: String,
    pub short_name: String,
    pub display_name: String,
    pub default_decimals: u8,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    pub logo: String,
    pub color: String,
    pub sort: u32,
    pub enabled: bool,

    // Specific
    pub chain_id: String,
    pub info: Option<ChainInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferBlockchainMeta {
    // Common
    pub name: String,
    pub short_name: String,
    pub display_name: String,
    pub default_decimals: u8,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    pub logo: String,
    pub color: String,
    pub sort: u32,
    pub enabled: bool,
    // Specific
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolanaBlockchainMeta {
    // Common
    pub name: String,
    pub short_name: String,
    pub display_name: String,
    pub default_decimals: u8,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    pub logo: String,
    pub color: String,
    pub sort: u32,
    pub enabled: bool,

    // Specific
    pub chain_id: String,
    // Always null
    pub info: Option<()>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarkNetBlockchainMeta {
    // Common
    pub name: String,
    pub short_name: String,
    pub display_name: String,
    pub default_decimals: u8,
    pub fee_assets: Vec<Asset>,
    pub address_patterns: Vec<String>,
    pub logo: String,
    pub color: String,
    pub sort: u32,
    pub enabled: bool,

    // Specific
    pub chain_id: String,
    pub info: ChainInfo,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum BlockchainMeta {
    EVM(EvmBlockchainMeta),
    TRANSFER(TransferBlockchainMeta),
    COSMOS(CosmosBlockchainMeta),
    SOLANA(SolanaBlockchainMeta),
    TRON(TronBlockchainMeta),
    STARKNET(StarkNetBlockchainMeta),
}
