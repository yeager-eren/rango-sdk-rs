use serde::{Deserialize, Serialize};

use crate::meta::swappers::{SwapperMeta, SwapperType};

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

#[derive(Serialize, Deserialize, Debug)]
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

impl Into<QuoteRequestQs> for QuoteRequest {
    fn into(self) -> QuoteRequestQs {
        let from = match self.from.address {
            None => format!("{}.{}", self.from.blockchain, self.from.symbol),
            Some(address) => format!("{}.{}--{}", self.from.blockchain, self.from.symbol, address),
        };
        let to = match self.to.address {
            None => format!("{}.{}", self.to.blockchain, self.to.symbol),
            Some(address) => format!("{}.{}--{}", self.to.blockchain, self.to.symbol, address),
        };

        return QuoteRequestQs {
            from,
            to,
            amount: self.amount,
            contract_call: self.contract_call,
            destination_contract: self.destination_contract,
            im_message: self.im_message,
            messaging_protocols: match self.messaging_protocols {
                Some(protocols) => Some(protocols.join(",")),
                None => None,
            },
            source_contract: self.source_contract,
            swapper_groups: match self.swapper_groups {
                Some(groups) => Some(groups.join(",")),
                None => None,
            },
            swappers: match self.swappers {
                Some(swappers) => Some(swappers.join(",")),
                None => None,
            },
            swappers_exclude: self.swappers_exclude,
            swappers_groups_exclude: self.swappers_groups_exclude,
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequestQs {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub swappers: Option<String>,
    pub swappers_exclude: Option<bool>,
    pub swapper_groups: Option<String>,
    pub swappers_groups_exclude: Option<bool>,
    pub messaging_protocols: Option<String>,
    pub source_contract: Option<String>,
    pub destination_contract: Option<String>,
    pub im_message: Option<String>,
    pub contract_call: Option<bool>,
}

impl TryInto<String> for QuoteRequestQs {
    type Error = serde_urlencoded::ser::Error;

    fn try_into(self) -> Result<String, serde_urlencoded::ser::Error> {
        serde_urlencoded::to_string(self)
    }
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
