use serde::{Deserialize, Serialize};

use crate::{
    error::SdkErr,
    quote::{Asset, QuoteSimulationResult, RoutingResultType},
};

use self::transaction::Transaction;

mod cosmos;
mod evm;
mod transaction;
mod transfer;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    pub from: Asset,
    pub to: Asset,
    pub amount: String,
    pub from_address: String,
    pub to_address: String,
    pub slippage: String,
    pub swappers: Option<Vec<String>>,
    pub swappers_exclude: Option<bool>,
    pub swapper_groups: Option<Vec<String>>,
    pub swappers_groups_exclude: Option<bool>,
    pub messaging_protocols: Option<Vec<String>>,
    pub source_contract: Option<String>,
    pub destination_contract: Option<String>,
    pub im_message: Option<String>,
    pub contract_call: Option<bool>,
    pub disable_estimate: Option<bool>,
    pub referrer_address: Option<String>,
    pub referrer_fee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SwapRequestQs {
    from: String,
    to: String,
    amount: String,
    from_address: String,
    to_address: String,
    slippage: String,
    swappers: Option<String>,
    swappers_exclude: Option<bool>,
    swapper_groups: Option<String>,
    swappers_groups_exclude: Option<bool>,
    messaging_protocols: Option<String>,
    source_contract: Option<String>,
    destination_contract: Option<String>,
    im_message: Option<String>,
    contract_call: Option<bool>,
    disable_estimate: Option<bool>,
    referrer_address: Option<String>,
    referrer_fee: Option<String>,
}

impl SwapRequest {
    pub fn into_qs(&self) -> Result<String, SdkErr> {
        let from = match self.from.address.clone() {
            None => format!("{}.{}", self.from.blockchain, self.from.symbol),
            Some(address) => format!("{}.{}--{}", self.from.blockchain, self.from.symbol, address),
        };
        let to = match self.to.address.clone() {
            None => format!("{}.{}", self.to.blockchain, self.to.symbol),
            Some(address) => format!("{}.{}--{}", self.to.blockchain, self.to.symbol, address),
        };

        let request = SwapRequestQs {
            from,
            to,
            amount: self.amount.clone(),
            from_address: self.from_address.clone(),
            to_address: self.to_address.clone(),
            slippage: self.slippage.clone(),
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
            disable_estimate: self.disable_estimate,
            referrer_address: self.referrer_address.clone(),
            referrer_fee: self.referrer_fee.clone(),
        };

        let qs = serde_urlencoded::to_string(request)?;

        Ok(qs)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    pub request_id: String,
    pub result_type: RoutingResultType,
    pub route: Option<QuoteSimulationResult>,
    pub error: Option<String>,
    pub tx: Option<Transaction>,
}
