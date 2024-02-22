use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosStakeCurrency {
    pub coin_denom: String,
    pub coin_minimal_denom: String,
    pub coin_decimals: u32,
    pub coin_gecko_id: String,
    pub coin_image_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBip44 {
    pub coin_type: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosBech32Config {
    pub bech32_prefix_acc_addr: String,
    pub bech32_prefix_acc_pub: String,
    pub bech32_prefix_val_addr: String,
    pub bech32_prefix_val_pub: String,
    pub bech32_prefix_cons_addr: String,
    pub bech32_prefix_cons_pub: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCurrency {
    pub coin_denom: String,
    pub coin_minimal_denom: String,
    pub coin_decimals: u32,
    pub coin_gecko_id: String,
    pub coin_image_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosGasPriceStep {
    pub low: f32,
    pub average: f32,
    pub high: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CosmosChainInfo {
    pub experimental: bool,
    pub rpc: String,
    pub rest: String,
    pub cosmostation_lcd_url: Option<String>,
    pub cosmostation_api_url: Option<String>,
    pub cosmostation_denom_trace_path: Option<String>,
    pub mint_scan_name: Option<String>,
    pub chain_name: String,
    pub stake_currency: CosmosStakeCurrency,
    pub bip44: CosmosBip44,
    pub bech32_config: CosmosBech32Config,
    pub currencies: Vec<CosmosCurrency>,
    pub fee_currencies: Vec<CosmosCurrency>,
    pub features: Vec<String>,
    pub explorer_url_to_tx: String,
    pub gas_price_step: Option<CosmosGasPriceStep>,
}
