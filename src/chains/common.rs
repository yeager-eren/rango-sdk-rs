use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    name: String,
    symbol: String,
    decimals: u8,
}
