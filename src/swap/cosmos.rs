use serde::Deserialize;

use super::transaction::AssetWithTicker;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoin {
    pub amount: String,
    pub denom: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignType {
    AMINO,
    DIRECT,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosProtoMsg {
    pub type_url: String,
    pub value: Vec<u8>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosFee {
    pub gas: String,
    pub amount: Vec<CosmosCoin>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosMessage {
    pub sign_type: SignType,
    pub sequence: Option<String>,
    pub source: Option<u32>,
    pub account_number: Option<u32>,
    pub rpc_url: String,
    pub chain_id: Option<String>,
    pub msgs: Vec<serde_json::Value>, // TODO: Update the type for msgs
    pub proto_msgs: Vec<CosmosProtoMsg>,
    pub memo: Option<String>,
    pub fee: Option<CosmosFee>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosRawTransferData {
    pub amount: String,
    pub asset: AssetWithTicker,
    pub decimals: u32,
    pub memo: Option<String>,
    pub method: String,
    pub recipient: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosTransaction {
    pub block_chain: String,
    pub from_wallet_address: String,
    pub data: CosmosMessage,
    pub raw_transfer: Option<CosmosRawTransferData>,
}
