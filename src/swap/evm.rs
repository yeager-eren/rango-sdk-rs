use serde::Deserialize;

use crate::meta::blockchain::EvmBlockchainMeta;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmTransaction {
    pub block_chain: EvmBlockchainMeta,
    pub from: Option<String>,
    pub approve_to: Option<String>,
    pub approve_data: Option<String>,
    pub tx_to: String,
    pub tx_data: Option<String>,
    pub value: Option<String>,
    pub gas_limit: Option<String>,
    pub gas_price: Option<String>,
    pub max_priority_fee_per_gas: Option<String>,
    pub max_fee_per_gas: Option<String>,
}
