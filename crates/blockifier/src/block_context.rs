use std::collections::HashMap;
use std::sync::Arc;

use starknet_api::block::{BlockNumber, BlockTimestamp};
use starknet_api::core::{ChainId, ContractAddress};
use starknet_api::hash::StarkFelt;
use starknet_api::transaction::TransactionVersion;

use crate::transaction::objects::HasRelatedFeeType;

#[derive(Clone, Debug)]
pub struct BlockContext {
    pub chain_id: ChainId,
    pub block_number: BlockNumber,
    pub block_timestamp: BlockTimestamp,

    // Fee-related.
    pub sequencer_address: ContractAddress,
    pub fee_token_addresses: FeeTokenAddresses,
    pub vm_resource_fee_cost: Arc<HashMap<String, f64>>,
    pub gas_prices: GasPrices,

    // Limits.
    pub invoke_tx_max_n_steps: u32,
    pub validate_max_n_steps: u32,
    pub max_recursion_depth: usize,
}

impl BlockContext {
    pub fn fee_token_address(&self, version: &dyn HasRelatedFeeType) -> ContractAddress {
        self.fee_token_addresses.get_for_version(version)
    }
}

#[derive(Clone, Debug)]
pub struct FeeTokenAddresses {
    pub strk_fee_token_address: ContractAddress,
    pub eth_fee_token_address: ContractAddress,
}

impl FeeTokenAddresses {
    pub fn get_for_version(&self, has_version: &dyn HasRelatedFeeType) -> ContractAddress {
        if is_strk_version(has_version) {
            self.strk_fee_token_address
        } else {
            self.eth_fee_token_address
        }
    }
}

#[derive(Clone, Debug)]
pub struct GasPrices {
    pub eth_l1_gas_price: u128, // In wei.
    // TODO(Amos, 01/09/2023): NEW_TOKEN_SUPPORT use this gas price for V3 txs.
    pub strk_l1_gas_price: u128, // In STRK.
}

impl GasPrices {
    pub fn get_for_version(&self, has_version: &dyn HasRelatedFeeType) -> u128 {
        if is_strk_version(has_version) { self.strk_l1_gas_price } else { self.eth_l1_gas_price }
    }
}

fn is_strk_version(has_version: &dyn HasRelatedFeeType) -> bool {
    !has_version.is_l1_handler()
        && has_version.version() >= TransactionVersion(StarkFelt::from(3_u128))
}
