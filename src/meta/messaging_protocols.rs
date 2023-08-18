use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingProtocol {
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingProtocols {
    pub protocols: Vec<MessagingProtocol>,
}
