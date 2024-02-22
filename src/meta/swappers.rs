use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SwapperType {
    Bridge,
    Dex,
    Aggregator,
    OffChain,
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
