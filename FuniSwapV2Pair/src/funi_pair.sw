contract;

mod events;

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

use ::events::{
    MintEvent,
    BurnEvent,
    SwapEvent,
};

/// The name of a specific asset minted by this contract.
const NAME: str[5] = __to_str_array("FuSV2");

/// The symbol of a specific asset minted by this contract.
const SYMBOL: str[3] = __to_str_array("FV2");

/// decimals
const DECIMALS: u8 = 9;

const MINIMUM_LIQUIDITY: u64 = 1000;

configurable {
    token0: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    token1: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}

storage {
    /// FuniSwapV2Pair ABI

    // we don't need the factory address storage, because we won't "initialize" the Pair 
    // contract. In UniswapV2 the initialize() callable by factory sets the token0 and token1.

    // reserves - deposits turned into liquidity.
    reserve0: u64 = 0,
    reserve1: u64 = 0,

    /// SRC20 ABI
    // The total number of distinguishable assets minted by this contract.
    total_assets: u64 = 1,
    // The total supply of coins for a specific asset minted by this contract.
    total_supply: u64 = 0,
}

abi FuniSwapV2Pair {
    #[storage(read)]
    fn get_reserves() -> (u64, u64);

    #[payable]
    #[storage(read, write)]
    fn burn(to: Identity) -> (u64, u64);

    #[storage(read, write)]
    fn mint(to: Identity) -> u64;

    #[storage(read, write)]
    fn swap(amount0_out: u64, amount1_out: u64, to: Identity);
}

impl FuniSwapV2Pair for Contract {
    #[storage(read)]
    fn get_reserves() -> (u64, u64) {
        _get_reserves()
    }

    #[storage(read, write)]
    fn mint(to: Identity) -> u64 {
        let total_supply = storage.total_supply.read();
        let mut liquidity = 0;
        let (reserve0, reserve1) = _get_reserves();
        let balance0 = this_balance(token0);
        let balance1 = this_balance(token1);
        let amount0 = balance0 - reserve0;
        let amount1 = balance1 - reserve1;

        if total_supply == 0 {
            liquidity = (amount0 * amount1).sqrt() - MINIMUM_LIQUIDITY;
            storage.total_supply.write(MINIMUM_LIQUIDITY); // instead of mint 1000 to 0
        } else {
            liquidity = _min(
                (amount0 * total_supply) / reserve0,
                (amount1 * total_supply) / reserve1
            )
        }
        require(liquidity > 0, "Insufficient Liquidity");
        _mint(to, liquidity);
        _update(balance0, balance1, reserve0, reserve1);

        log(MintEvent{
            sender: msg_sender().unwrap(),
            to,
            amount0,
            amount1,
        });

        liquidity
    }

    #[payable]
    #[storage(read, write)]
    fn burn(to: Identity) -> (u64, u64) {
        let total_supply = storage.total_supply.read();
        let liquidity = msg_amount();
        let (reserve0, reserve1) = _get_reserves();
        let balance0 = this_balance(token0);
        let balance1 = this_balance(token1);

        let amount0 = (liquidity * balance0) / total_supply;
        let amount1 = (liquidity * balance1) / total_supply;
        require(amount0 > 0 && amount1 > 0, "Insufficient Liquidity Burned");
        _burn(liquidity);
        transfer(to, token0, amount0);
        transfer(to, token1, amount1);

        _update(
            this_balance(token0),
            this_balance(token1),
            reserve0,
            reserve1
        );

        log(BurnEvent{
            sender: msg_sender().unwrap(),
            to,
            amount0,
            amount1,
        });

        (amount0, amount1)
    }

    #[storage(read, write)]
    fn swap(amount0_out: u64, amount1_out: u64, to: Identity) {
        require(amount0_out > 0 || amount1_out > 0, "Insufficient Output Amount");
        let (reserve0, reserve1) = _get_reserves();
        require(amount0_out < reserve0 && amount1_out < reserve1, "Insufficient Liquidity");
        // do we need to check if we don't transfer tokens to token0 or token1 contracts?

        if amount0_out > 0 {
            transfer(to, token0, amount0_out);
        }
        if amount1_out > 0 {
            transfer(to, token1, amount1_out);
        }
        let balance0 = this_balance(token0);
        let balance1 = this_balance(token1);

        let mut amount0_in = 0;
        let mut amount1_in = 0;

        if balance0 > reserve0 - amount0_out {
            amount0_in = balance0 - (reserve0 - amount0_out);
        }
        if balance1 > reserve1 - amount1_out {
            amount1_in = balance1 - (reserve1 - amount1_out);
        }
        require(amount0_in > 0 || amount1_in > 0, "Insufficient Input Amount");

        let balance0_adjusted = (balance0 * 1000) - (amount0_in * 3);
        let balance1_adjusted = (balance1 * 1000) - (amount1_in * 3);
        require(
            balance0_adjusted * balance1_adjusted >= reserve0 * reserve1 * 1000000,
            "K Invariant Incorrect"
        );
        _update(balance0, balance1, reserve0, reserve1);

        log(SwapEvent{
            sender: msg_sender().unwrap(),
            to,
            amount0_in,
            amount1_in,
            amount0_out,
            amount1_out,
        });
    }
}

abi SRC20 {
    #[storage(read)]
    fn total_assets() -> u64;

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64>;

    fn name(asset: AssetId) -> Option<String>;

    fn symbol(asset: AssetId) -> Option<String>;

    fn decimals(asset: AssetId) -> Option<u8>;
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

    fn name(asset: AssetId) -> Option<String> {
        if asset == AssetId::default() {
            Some(String::from_ascii_str(from_str_array(NAME)))
        } else {
            None
        }
    }

    fn symbol(asset: AssetId) -> Option<String> {
        if asset == AssetId::default() {
            Some(String::from_ascii_str(from_str_array(SYMBOL)))
        } else {
            None
        }
    }

    fn decimals(asset: AssetId) -> Option<u8> {
        if asset == AssetId::default() {
            Some(DECIMALS)
        } else {
            None
        }
    }
}

#[storage(read, write)]
fn _mint(recipient: Identity, amount: u64) {
    // Increment total supply of the asset and mint to the recipient.
    storage.total_supply.write(amount + storage.total_supply.read());
    mint_to(recipient, DEFAULT_SUB_ID, amount);
}

#[storage(read, write)]
fn _burn(amount: u64) {
    require(
        msg_asset_id() == AssetId::default(),
        "Incorrect asset provided",
    );
 
    // Decrement total supply of the asset and burn.
    storage.total_supply.write(storage.total_supply.read() - amount);
    burn(DEFAULT_SUB_ID, amount);
}

fn _min(a: u64, b: u64) -> u64 {
    if a >= b {
        a
    } else {
        b
    }
}

#[storage(read)]
fn _get_reserves() -> (u64, u64) {
    (
        storage.reserve0.read(),
        storage.reserve1.read()
    )
}

#[storage(read, write)]
fn _update(balance0: u64, balance1: u64, reserve0: u64, reserve1: u64) {
    storage.reserve0.write(balance0);
    storage.reserve1.write(balance1);
}

