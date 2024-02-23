use crate::{
    error::SdkErr,
    meta::{
        blockchains::BlockchainMeta, messaging_protocols::MessagingProtocols, meta::Meta,
        swappers::SwapperMeta,
    },
};

impl super::Client {
    /// All the essential data needed for a client, including list of support blockchains, tokens and protocols (DEXs & Bridges).
    pub async fn meta(&self) -> Result<Meta, SdkErr> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta", self.config.api_key
        );

        println!("url: {:?}", url);

        let body: Meta = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }

    /// List of supported chains by Rango
    pub async fn chains(&self) -> Result<Vec<BlockchainMeta>, SdkErr> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta/blockchains", self.config.api_key
        );

        let body: Vec<BlockchainMeta> = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }

    /// List of supported swappers by Rango
    pub async fn swappers(&self) -> Result<Vec<SwapperMeta>, SdkErr> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta/swappers", self.config.api_key
        );

        let body: Vec<SwapperMeta> = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }

    /// List of supported messaging protocols by Rango
    pub async fn messaging_protocols(&self) -> Result<MessagingProtocols, SdkErr> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta/messaging-protocols", self.config.api_key
        );

        let body: MessagingProtocols = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}
