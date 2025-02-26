contract;

mod errors;
mod events;

use ::errors::InitError;
use ::events::{RegisterPoolEvent, SetExchangeBytecodeRootEvent};

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
    external::bytecode_root, 
    hash::Hash,
};

storage {
    /* the pair contract bytecode root */
    pair_bytecode_root: Option<b256> = Option::None,

    /* Map that stores all pool contracts */
    pairs: StorageMap<(AssetId, AssetId), ContractId> = StorageMap {},
}

abi FuniSwapV2Factory {
    #[storage(read, write)]
    fn initialize(pair_bytecode_root: ContractId);

    #[storage(read, write)]
    fn create_pair(_token0: AssetId, _token1: AssetId, pair: ContractId);
}

impl FuniSwapV2Factory for Contract {
    #[storage(read, write)]
    fn initialize(pair_contract: ContractId) {
        require(
            storage
                .pair_bytecode_root
                .read()
                .is_none(),
            InitError::BytecodeRootAlreadySet,
        );

        let pair_bytecode_root = bytecode_root(pair_contract);
        storage
            .pair_bytecode_root
            .write(Option::Some(pair_bytecode_root));
        log(SetExchangeBytecodeRootEvent {
            root: pair_bytecode_root,
        });
    }

    #[storage(read, write)]
    fn create_pair(_token0: AssetId, _token1: AssetId, pair: ContractId) {
        require(_token0 != _token1, "Identical AssetIds");
        let mut token0 = _token0;
        let mut token1 = _token1;
        if token0.bits() > token1.bits() {
            token0 = _token1;
            token1 = _token0;
        }

        /* verify if factory is initialized */
        require(
            storage
                .pair_bytecode_root
                .read()
                .is_some(),
            InitError::BytecodeRootNotSet,
        );

        /* verify if the bytecode matches the desired pair implementation */
        require(
            storage
                .pair_bytecode_root
                .read()
                .unwrap() == bytecode_root(pair),
            InitError::BytecodeRootDoesNotMatch,
        );

    }
}
