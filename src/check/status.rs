use serde::{Deserialize, Serialize};

use crate::{error::SdkErr, quote::Token};

use super::is_approved::TransactionStatus;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusRequest {
    pub request_id: String,
    pub tx_id: String,
}

impl StatusRequest {
    pub fn into_qs(&self) -> Result<String, SdkErr> {
        let qs: String = serde_urlencoded::to_string(self)?;
        Ok(qs)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatusOutputType {
    RevertedToInput,
    MiddleAssetInSrc,
    MiddleAssetInDest,
    DesiredOutput,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusOutput {
    pub amount: String,
    pub received_token: Token,
    pub r#type: StatusOutputType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapExplorerUrl {
    pub description: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeData {
    pub src_chain_id: u32,
    pub src_tx_hash: Option<String>,
    pub src_token: Option<String>,
    pub src_token_amt: String,
    pub src_token_decimals: u32,
    pub src_token_price: Option<String>,
    pub dest_chain_id: u32,
    pub dest_tx_hash: Option<String>,
    pub dest_token: Option<String>,
    pub dest_token_amt: Option<String>,
    pub dest_token_decimals: u32,
    pub dest_token_price: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub status: Option<TransactionStatus>,
    pub error: Option<String>,
    pub output: Option<StatusOutput>,
    pub explorer_url: Option<Vec<SwapExplorerUrl>>,
    pub bridge_data: Option<BridgeData>,
}
