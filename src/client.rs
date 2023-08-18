use std::rc::Rc;

pub use crate::meta::MetaClient;

const API_URL: &str = "https://api.rango.exchange";

/// Configs for your client (e.g API Key)
pub struct ClientConfig {
    pub device_id: String,
    pub api_key: String,
    pub api_url: String,
}

/// Client for interacting with Rango APIs
pub struct Client {
    pub meta: MetaClient,
}

impl Client {
    pub fn new(device_id: &str, api_key: &str, api_url: Option<String>) -> Self {
        let api_url = api_url.unwrap_or(API_URL.to_owned());

        let config = Rc::new(ClientConfig {
            api_url,
            device_id: device_id.into(),
            api_key: api_key.into(),
        });

        let meta = MetaClient {
            config: Rc::clone(&config),
        };

        Self { meta }
    }
}
