use std::rc::Rc;

use crate::client::ClientConfig;

use self::{
    blockchain::BlockchainMeta, messaging_protocols::MessagingProtocols, swappers::SwapperMeta,
};

mod blockchain;
mod messaging_protocols;
mod swappers;

/// Working with metadata to receive general information about blockchains, swappers,...
pub struct MetaClient {
    pub config: Rc<ClientConfig>,
}

impl MetaClient {
    /// List of supported blockchains by Rango with some general information   
    pub async fn chains(self) -> Result<Vec<BlockchainMeta>, ureq::Error> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta/blockchains", self.config.api_key
        );

        let body: Vec<BlockchainMeta> = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }

    /// List of supported swappers by Rango
    pub async fn swappers(self) -> Result<Vec<SwapperMeta>, ureq::Error> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta/swappers", self.config.api_key
        );

        let body: Vec<SwapperMeta> = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }

    /// List of supported messaging protocols by Rango
    pub async fn messaging_protocols(self) -> Result<MessagingProtocols, ureq::Error> {
        let url = format!(
            "{}/{}?apiKey={}",
            self.config.api_url, "basic/meta/messaging-protocols", self.config.api_key
        );

        let body: MessagingProtocols = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}
