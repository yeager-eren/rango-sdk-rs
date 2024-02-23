use crate::{
    error::SdkErr,
    swap::{SwapRequest, SwapResponse},
};

impl super::Client {
    /// After getting a quote, you will call swap. it includes the route and required transaction to be singed by user.
    pub async fn swap(&self, request: SwapRequest) -> Result<SwapResponse, SdkErr> {
        let qs = request.into_qs()?;

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/swap", self.config.api_key, qs
        );

        let body: SwapResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}
