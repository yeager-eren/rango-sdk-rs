use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum SwapperType {
    BRIDGE,
    DEX,
    AGGREGATOR,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapperMeta {
    pub id: String,
    pub title: String,
    pub logo: String,
    pub swapper_group: String,
    pub types: Vec<SwapperType>,
}
