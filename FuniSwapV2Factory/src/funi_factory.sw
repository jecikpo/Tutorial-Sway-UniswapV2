contract;

use std::{
    asset::{
        burn,
        mint_to,
        transfer,
    },
    call_frames::msg_asset_id,
    context::msg_amount,
    context::this_balance,
    constants::DEFAULT_SUB_ID,
    string::String,
    storage::*,
    storage::storage_api::{
        read, 
        write
    },
    storage::storage_map::*,
    hash::*,
    asset_id::*,
};

pub const IDENTICAL_ADDRESSES_SIGNAL = 0xffff_ffff_fffe_0000;

abi FuniSwapV2Factory {
    fn create_pair(token0: AssetId, token1: AssetId) -> ContractId;
}

impl FuniSwapV2Factory for Contract {
    fn create_pair(_token0: AssetId, _token1: AssetId) -> ContractId {
        require(_token0 != _token1, "Identical AssetIds");
        let mut token0 = _token0;
        let mut token1 = _token1;
        if token0.bits() > token1.bits {
            token0 = _token1;
            token1 = _token0;
        }
    }
}
