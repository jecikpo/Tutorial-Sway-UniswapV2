contract;

use std::{
    asset::{
        burn,
        mint_to,
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

use std::{bytes::Bytes, bytes_conversions::{b256::*, u16::*, u256::*, u32::*, u64::*,}};

configurable {
    /// The name of a specific asset minted by this contract.
    name: str[5] = __to_str_array("Token"),
    /// The symbol of a specific asset minted by this contract.
    symbol: str[3] = __to_str_array("TKN"),
    /// The decimals of a specific asset minted by this contract.
    decimals: u8 = 9,
}


abi SRC20 {
    #[storage(read)]
    fn total_assets() -> u64;

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64>;

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String>;

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String>;

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8>;

    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64);

    #[payable]
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64);
}


storage {
    /// The total number of distinguishable assets minted by this contract.
    total_assets: u64 = 1,
    /// The total supply of coins for a specific asset minted by this contract.
    total_supply: u64 = 0,
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        storage.total_assets.read()
    }

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        if asset == AssetId::default() {
            Some(storage.total_supply.read())
        } else {
            None
        }
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        if asset == AssetId::default() {
            Some(String::from_ascii_str(from_str_array(name)))
        } else {
            None
        }
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        if asset == AssetId::default() {
            Some(String::from_ascii_str(from_str_array(symbol)))
        } else {
            None
        }
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        if asset == AssetId::default() {
            Some(decimals)
        } else {
            None
        }
    }

    #[storage(read, write)]
    fn mint(recipient: Identity, sub_id: SubId, amount: u64) {
        require(sub_id == DEFAULT_SUB_ID, "Incorrect Sub Id");
 
        // Increment total supply of the asset and mint to the recipient.
        storage.total_supply.write(amount + storage.total_supply.read());
        mint_to(recipient, DEFAULT_SUB_ID, amount);
    }

    #[payable]
    #[storage(read, write)]
    fn burn(sub_id: SubId, amount: u64) {
        require(sub_id == DEFAULT_SUB_ID, "Incorrect Sub Id");
        require(msg_amount() >= amount, "Incorrect amount provided");
        require(
            msg_asset_id() == AssetId::default(),
            "Incorrect asset provided",
        );
 
        // Decrement total supply of the asset and burn.
        storage.total_supply.write(storage.total_supply.read() - amount);
        burn(DEFAULT_SUB_ID, amount);
    }
}



