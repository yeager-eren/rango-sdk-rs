const API_URL: &str = "https://api.rango.exchange";

/// Configs for your client (e.g API Key)
struct ClientConfig {
    #[allow(dead_code)]
    device_id: String,
    api_key: String,
    api_url: String,
}

/// Client for interacting with Rango APIs
pub struct Client {
    config: ClientConfig,
}

impl Client {
    /// Creating a client to send request to Rango's server using available methods.
    pub fn new(device_id: &str, api_key: &str, api_url: Option<String>) -> Self {
        let api_url = api_url.unwrap_or(API_URL.to_owned());

        Self {
            config: ClientConfig {
                api_url,
                device_id: device_id.into(),
                api_key: api_key.into(),
            },
        }
    }
}

mod check;
mod meta;
// mod misc;
mod quote;
mod swap;
