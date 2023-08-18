use serde::Deserialize;

use self::{
    cosmos::CosmosChainInfo, evm::EvmChainInfo, starknet::StarkNetChainInfo, tron::TronChainInfo,
};

mod common;
mod cosmos;
mod evm;
mod starknet;
mod tron;

#[derive(Deserialize, Debug)]
#[serde(tag = "infoType")]
pub enum ChainInfo {
    EvmMetaInfo(EvmChainInfo),
    CosmosMetaInfo(CosmosChainInfo),
    StarkNetMetaInfo(StarkNetChainInfo),
    TronMetaInfo(TronChainInfo),
}
