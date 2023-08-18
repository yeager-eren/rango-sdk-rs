use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CosmosStakeCurrency {
    coin_denom: String,
    coin_minimal_denom: String,
    coin_decimals: u32,
    coin_gecko_id: String,
    coin_image_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CosmosBip44 {
    coin_type: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CosmosBech32Config {
    bech32_prefix_acc_addr: String,
    bech32_prefix_acc_pub: String,
    bech32_prefix_val_addr: String,
    bech32_prefix_val_pub: String,
    bech32_prefix_cons_addr: String,
    bech32_prefix_cons_pub: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CosmosCurrency {
    coin_denom: String,
    coin_minimal_denom: String,
    coin_decimals: u32,
    coin_gecko_id: String,
    coin_image_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CosmosGasPriceStep {
    low: f32,
    average: f32,
    high: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosChainInfo {
    experimental: bool,
    rpc: String,
    rest: String,
    cosmostation_lcd_url: Option<String>,
    cosmostation_api_url: Option<String>,
    cosmostation_denom_trace_path: Option<String>,
    mint_scan_name: Option<String>,
    chain_name: String,
    stake_currency: CosmosStakeCurrency,
    bip44: CosmosBip44,
    bech32_config: CosmosBech32Config,
    currencies: Vec<CosmosCurrency>,
    fee_currencies: Vec<CosmosCurrency>,
    features: Vec<String>,
    explorer_url_to_tx: String,
    gas_price_step: Option<CosmosGasPriceStep>,
}
