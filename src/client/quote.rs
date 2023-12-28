use crate::{
    error::SdkErr,
    quote::{QuoteRequest, QuoteResponse},
};

impl super::Client {
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
