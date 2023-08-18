use serde::Deserialize;

use super::common::NativeCurrency;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StarkNetChainInfo {
    pub chain_name: String,
    pub native_currency: NativeCurrency,
    pub block_explorer_urls: Vec<String>,
    pub address_url: String,
    pub transaction_url: String,
}
