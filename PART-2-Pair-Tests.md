# Funiswap Pair Tests

In this part of the tutorial we will create some tests the FuniswapV2Pair contract 
that was written in [part 1](https://github.com/jecikpo/Tutorial-Sway-UniswapV2/blob/main/PART-1-Pair.md).

We will use the same [framework](https://github.com/jecikpo/Tutorial-Fuel-SRC20/?tab=readme-ov-file#testing-framework-overview) for writing tests as we did in the SRC20 tutorial.

# Preparing the framework

To prepare the framework, you need to copy the following test files into your `test` directory:
```bash
tests/harness.rs
tests/utils/funi_pair.rs
tests/utils/instance.rs
tests/utils/mod.rs
tests/utils/setup.rs
tests/utils/src20.rs
tests/funi_pair/mod.rs
```

We will also need our SRC20 contract that we will leverage to make swaps. Copy the 
entire `SRC20` [directory](https://github.com/jecikpo/Tutorial-Sway-UniswapV2/tree/main/SRC20) to your project folder.

Make sure that your `Forc.toml` in your main project dir points the the two contracts that we will be building,
`FuniSwapV2Pair` and `SRC20`:
```conf
[workspace]
members = [
    "./SRC20",
    "./FuniSwapV2Pair"
]
```
Your `tests/harness.rs` should only contain the following lines:
```rust
mod utils;
mod src20;
mod funi_pair;
```
The `tests/utils/funi_pair.rs` file already contains the necessary wrappers for the methods of our pair contract. The
actual tests we will be writing in the `tests/funi_pair/liquidity.rs`. Create that file and put the following
code in it:
```rust
use crate::utils::setup::*;
use crate::utils::funi_pair::*;
use crate::utils::src20::*;
use crate::utils::instance::*;

use fuels::{
    prelude::*,
    types::{
        AssetId,
        bech32::Bech32ContractId,
        Identity,
    }
};
```
Now you should be able to build both, the pair and SRC20 contracts:
```bash
forc build
```

If it succeeds we are ready to write our first test.

# Testing Minting
We will start building our tests from providing initial liquidity into the pool. We will create the empty test function
in `tests/funi_pair/liquidity.rs`:

```rust
#[tokio::test]
async fn test_funi_mint_initial() {

}
```

We will deploy two SRC20 contracts that will emulate our exchangable tokens:
```rust
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
```

Next we will deploy the pair contract and we will configure it using the above Asset Ids. As you remember the 
pair contract's Asset Ids are within the `configurable` block, so we need to provide them during in the deployment TX:
```rust
    let funi_pair_configurables = create_funi_pair_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let pair = ContractInstance::<FuniSwapV2Pair<WalletUnlocked>>::new_with_configurables(funi_pair_configurables).await;
```

Now, let's get the depositor's Identity, define the amounts of liquidity that we will deploy and the amount of LP coins 
that we are going to get minted by our pair contract:
```rust
    let depositor = get_deployer_identity().await;
    let amount0 = 1000;
    let amount1 = 10000;
    let expected_liquidity = 2162;
```

The value `2162` comes from the calculation: 
```rust
liquidity = (amount0 * amount1).sqrt() - MINIMUM_LIQUIDITY;
```

Now we need to mint our SRC20 assets to the `depositor` and transfer them to the pair contract. Remember that because
of how Fuel handles SRC20 coins as native assets it was not possible to pull them by the contract from the respective
owner, hence we have to push them manually. Of course this method is not safe, as in the real world, someone could
front-run our `mint()` call after we transfered assets and mint the LP token for themselves. We will handle that problem
in later parts of the tutorial.

```rust
    /* mint some token0 and token1 for the depositor */
    token0.clone().call_mint(depositor, DEFAULT_SUB_ID, amount0).await;
    token1.clone().call_mint(depositor, DEFAULT_SUB_ID, amount1).await;

    /* transfer assets to pair contract */
    let wallet = pair.deployer_wallet();
    wallet.transfer_to_contract(
        pair.contract_id(),
        token0_asset_id,
        amount0
    ).await;
    wallet.transfer_to_contract(
        pair.contract_id(),
        token1_asset_id,
        amount1
    ).await;
```
The `wallet` represents here the Fuel Wallet object which has the `depositor` Identity.

We are ready to deploy our liquidity and verify the return value:
```rust
    let liquidity = pair.clone().call_mint(depositor).await;
    assert_eq!(expected_liquidity, liquidity);
```

We should also validate if we indeed received our LP tokens in the expected amount:

```rust
    let pair_balance = pair.clone()
        .deployer_balance(
            get_default_asset_id(
                pair.contract_id()
            )
        ).await;

    assert_eq!(
        pair_balance, 
        expected_liquidity
    )
```

We are done with our test function. Now you should be able to run the test with the following command:
```bash
cargo test
```
And the output should give you:
```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 17s
    Running tests/harness.rs (target/debug/deps/integration_tests-fdc4356cf09b9334)

running 1 tests
test funi_pair::liquidity::test_funi_mint_initial ... ok
```

# Testing Burning
In this section we will write a test for withdrawing an LP position. Let's start with an empty function:
```rust
#[tokio::test]
async fn test_funi_burn_initial() {

}
```

Let's start by defining the amounts of tokens that are going in during minting and the expected amounts
that we are expecting to get back once the LP tokens are burned:
```rust
    let amount0 = 1000;
    let amount1 = 10000;
    let expected_amount0_out = 683;
    let expected_amount1_out = 6837;
```
We are getting less out for burning the initial amount because of the UniswapV2's protection against the 
"Inflation Attack". Details of this attack can be found in the article on [ERC 4626](https://www.rareskills.io/post/erc4626).

To test burning we need to have a pair contract with some liquidity deployed. I wrote a wrapper to enclose this,
so that our test function maintains it's readability:
```rust
    let (token0, token1, pair, liquidity) = setup_funi_pair(
        amount0,
        amount1,
    ).await;
```

Before calling `burn()` we will record our SRC20 balances:
```rust
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let token0_balance_before = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_before = pair.clone().deployer_balance(token1_asset_id).await;
```

and call `burn()` finally:
```rust
    let (received_token0, received_token1) = pair.clone().call_burn(
        pair.deployer_identity(),
        liquidity,
    ).await;
```

Now let's record the balances again: 
```rust
    let token0_balance_after = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_after = pair.clone().deployer_balance(token1_asset_id).await;
```

And verify if the difference in balances is as expected and the function return correct values 
of returned tokens:
```rust
    assert_eq!(expected_amount0_out, token0_balance_after - token0_balance_before);
    assert_eq!(expected_amount1_out, token1_balance_after - token1_balance_before);

    assert_eq!(expected_amount0_out, received_token0);
    assert_eq!(expected_amount1_out, received_token1);
```

We are done with testing burning of the initial LP position.