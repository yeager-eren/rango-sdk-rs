use serde::Deserialize;

use super::common::NativeCurrency;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EvmChainInfo {
    chain_name: String,
    native_currency: NativeCurrency,
    rpc_urls: Vec<String>,
    block_explorer_urls: Vec<String>,
    address_url: String,
    transaction_url: String,
}
