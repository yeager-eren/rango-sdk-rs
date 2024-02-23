use crate::{
    error::SdkErr,
    quote::{QuoteRequest, QuoteResponse},
};

impl super::Client {
    /// Getting a quote for requested swap
    /// Rango will find the best route from available DEX or bridges for doing the swap. Quote is a preview of route, if the quote confirmed by the user, you will need to use `swap` for getting the actual swap and complete the swap.
    pub async fn quote(&self, request: QuoteRequest) -> Result<QuoteResponse, SdkErr> {
        let qs = request.into_qs()?;

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/quote", self.config.api_key, qs
        );

        let body: QuoteResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}
