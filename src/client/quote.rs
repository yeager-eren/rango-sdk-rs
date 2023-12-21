use crate::quote::{QuoteRequest, QuoteRequestQs, QuoteResponse};

impl super::Client {
    pub async fn quote(&self, request: QuoteRequest) -> Result<QuoteResponse, ureq::Error> {
        let qs: QuoteRequestQs = request.into();
        // TODO: Don't using unwrap.
        let qs: String = qs.try_into().unwrap();

        let url = format!(
            "{}/{}?apiKey={}&{}",
            self.config.api_url, "basic/quote", self.config.api_key, qs
        );

        println!("url: {}", url);

        let body: QuoteResponse = ureq::get(&url).call()?.into_json()?;
        Ok(body)
    }
}