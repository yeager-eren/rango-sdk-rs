use crate::{
    error::SdkErr,
    meta::{
        blockchain::BlockchainMeta, messaging_protocols::MessagingProtocols, swappers::SwapperMeta,
    },
};

impl super::Client {
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
