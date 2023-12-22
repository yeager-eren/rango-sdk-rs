use serde::Deserialize;

// use self::{
//     cosmos::CosmosChainInfo, evm::EvmChainInfo, starknet::StarkNetChainInfo, tron::TronChainInfo,
// };

pub mod common;
pub mod cosmos;
pub mod evm;
pub mod starknet;
pub mod tron;

#[derive(Deserialize, Debug)]
#[serde(tag = "infoType")]
pub enum ChainInfo {
    EvmMetaInfo(evm::EvmChainInfo),
    CosmosMetaInfo(cosmos::CosmosChainInfo),
    StarkNetMetaInfo(starknet::StarkNetChainInfo),
    TronMetaInfo(tron::TronChainInfo),
}
