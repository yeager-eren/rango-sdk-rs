use serde::{Deserialize, Serialize};

use crate::error::SdkErr;

#[derive(Debug, Deserialize)]
#[serde[rename_all = "lowercase"]]
pub enum TransactionStatus {
    Failed,
    Running,
    Success,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckApprovalResponse {
    pub is_approved: bool,
    pub tx_status: Option<TransactionStatus>,
    pub required_approved_amount: Option<String>,
    pub current_approved_amount: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsApprovedRequest {
    pub request_id: String,
    pub tx_id: Option<String>,
}

impl IsApprovedRequest {
    pub fn into_qs(&self) -> Result<String, SdkErr> {
        let qs = serde_urlencoded::to_string(self)?;

        Ok(qs)
    }
}
