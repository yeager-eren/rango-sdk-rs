use serde::{Deserialize, Serialize};

use crate::{
    error::SdkErr,
    meta::swappers::{SwapperMeta, SwapperType},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub blockchain: String,
    pub chain_id: Option<String>,
    pub address: Option<String>,
    pub symbol: String,
    pub name: Option<String>,
    pub decimals: u32,
    pub image: String,
    pub blockchain_image: String,
    pub usd_price: Option<f64>,
    pub is_popular: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub blockchain: String,
    pub address: Option<String>,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    pub from: Asset,
    pub to: Asset,
    pub amount: String,
    pub swappers: Option<Vec<String>>,
    pub swappers_exclude: Option<bool>,
    pub swapper_groups: Option<Vec<String>>,
    pub swappers_groups_exclude: Option<bool>,
    pub messaging_protocols: Option<Vec<String>>,
    pub source_contract: Option<String>,
    pub destination_contract: Option<String>,
    pub im_message: Option<String>,
    pub contract_call: Option<bool>,
}

impl QuoteRequest {
    pub fn into_qs(&self) -> Result<String, SdkErr> {
        let from = match self.from.address.clone() {
            None => format!("{}.{}", self.from.blockchain, self.from.symbol),
            Some(address) => format!("{}.{}--{}", self.from.blockchain, self.from.symbol, address),
        };
        let to = match self.to.address.clone() {
            None => format!("{}.{}", self.to.blockchain, self.to.symbol),
            Some(address) => format!("{}.{}--{}", self.to.blockchain, self.to.symbol, address),
        };

        let request = QuoteRequestQs {
            from,
            to,
            amount: self.amount.clone(),
            contract_call: self.contract_call,
            destination_contract: self.destination_contract.clone(),
            im_message: self.im_message.clone(),
            source_contract: self.source_contract.clone(),
            messaging_protocols: match self.messaging_protocols.clone() {
                Some(protocols) => Some(protocols.join(",")),
                None => None,
            },
            swapper_groups: match self.swapper_groups.clone() {
                Some(groups) => Some(groups.join(",")),
                None => None,
            },
            swappers: match self.swappers.clone() {
                Some(swappers) => Some(swappers.join(",")),
                None => None,
            },
            swappers_exclude: self.swappers_exclude,
            swappers_groups_exclude: self.swappers_groups_exclude,
        };

        let qs = serde_urlencoded::to_string(request)?;

        Ok(qs)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QuoteRequestQs {
    from: String,
    to: String,
    amount: String,
    swappers: Option<String>,
    swappers_exclude: Option<bool>,
    swapper_groups: Option<String>,
    swappers_groups_exclude: Option<bool>,
    messaging_protocols: Option<String>,
    source_contract: Option<String>,
    destination_contract: Option<String>,
    im_message: Option<String>,
    contract_call: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoutingResultType {
    Ok,
    HighImpact,
    NoRoute,
    InputLimitIssue,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub request_id: String,
    pub result_type: RoutingResultType,
    pub route: Option<QuoteSimulationResult>,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteSimulationResult {
    pub from: Token,
    pub to: Token,
    pub output_amount: String,
    pub output_amount_min: String,
    pub output_amount_usd: Option<f64>,
    pub swapper: SwapperMeta,
    pub path: Option<Vec<QuotePath>>,
    pub fee: Vec<SwapFee>,
    pub fee_usd: Option<f64>,
    pub amount_restriction: Option<AmountRestriction>,
    pub estimated_time_in_seconds: u64,
}

#[derive(Deserialize, Debug)]
pub enum AmountRestrictionTypes {
    INCLUSIVE,
    EXCLUSIVE,
}

#[derive(Deserialize, Debug)]
pub struct AmountRestriction {
    pub min: Option<String>,
    pub max: Option<String>,
    pub r#type: AmountRestrictionTypes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExpenseType {
    FromSourceWallet,
    DecreaseFromOutput,
    FromDestinationWallet,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapFee {
    pub name: String,
    pub token: Token,
    pub expense_type: ExpenseType,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuotePath {
    pub from: Token,
    pub to: Token,
    pub swapper_type: SwapperType,
    pub swapper: SwapperMeta,
    pub expected_output: String,
    pub estimated_time_in_seconds: u64,
}
