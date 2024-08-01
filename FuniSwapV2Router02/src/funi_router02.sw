contract;

use std::{
    asset::{
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

/// Errors
pub const INCORRECT_ASSET_ID_SIGNAL = 0xffff_ffff_ffff_0000;

configurable {
    token0: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    token1: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}

abi FuniSwapV2Router02 {
    #[payable]
    #[storage(read, write)]
    fn deposit(to: Identity);

    #[storage(read, write)]
    fn withdraw(to: Identity);

    #[storage(read)]
    fn get_deposits(depositor: Identity) -> (u64, u64);
}

impl FuniSwapV2Router02 for Contract {
    #[payable]
    #[storage(read, write)]
    fn deposit(to: Identity) {
        let asset_id = msg_asset_id();
        let (token0_deposit, token1_deposit) = _get_deposits(to);

        if asset_id == token0 {
            _update_deposits(to, Some(msg_amount() + token0_deposit), None);
        } else if asset_id == token1 {
            _update_deposits(to, None, Some(msg_amount() + token1_deposit));
        } else {
            revert(INCORRECT_ASSET_ID_SIGNAL);
        }
    }

    #[storage(read, write)]
    fn withdraw(to: Identity) {
        let sender = msg_sender().unwrap();
        let (token0_deposit, token1_deposit) = _get_deposits(sender);
        require(token0_deposit > 0 || token1_deposit > 0, "No deposits");
        if token0_deposit > 0 {
            _update_deposits(sender, Some(0), None);
            transfer(to, token0, token0_deposit);
        }
        if token1_deposit > 0 {
            _update_deposits(sender, None, Some(0));
            transfer(to, token1, token1_deposit);
        }
    }

    #[storage(read)]
    fn get_deposits(depositor: Identity) -> (u64, u64) {
        _get_deposits(depositor)
    }
}

#[storage(read)]
fn _get_deposits(depositor: Identity) -> (u64, u64) {
    let token0_deposits = _get_token_0_deposits();
    let token0_existing_deposit = token0_deposits.get(depositor).try_read().unwrap_or(0);
    let token1_deposits = _get_token_1_deposits();
    let token1_existing_deposit = token1_deposits.get(depositor).try_read().unwrap_or(0);
    (token0_existing_deposit, token1_existing_deposit)
}

#[storage(read, write)]
fn _update_deposits(depositor: Identity, amount0: Option<u64>, amount1: Option<u64>) {
    if amount0 != None {
        let token0_deposits = _get_token_0_deposits();
        token0_deposits.insert(depositor, amount0.unwrap());
    }
    if amount1 != None {
        let token1_deposits = _get_token_1_deposits();
        token1_deposits.insert(depositor, amount1.unwrap());
    }
}

fn _get_token_0_deposits() -> StorageKey::<StorageMap<Identity, u64>> {
    StorageKey::<StorageMap<Identity, u64>>::new(b256::zero(), 0, token0.bits())
}

fn _get_token_1_deposits() -> StorageKey::<StorageMap<Identity, u64>> {
    StorageKey::<StorageMap<Identity, u64>>::new(b256::zero(), 0, token1.bits())
}