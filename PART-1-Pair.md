# Funiswap Pair Contract

In this part of the tutorial we will focus only on writing the FuniswapV2Pair contract 
without the Oracle (we will add it in later parts).

The contract written here is based on the `UniswapV2Pair.sol`.

## Introduction

The UniswapV2 Pair contract is part of the v2-core repo and its code can be found [here](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol).
We are using the same equations for calculating swaps and LP tokens to assets and vice-versa
as the original UniswapV2 code. 

I will use the terms coins and tokens interchangably as well as pair and pool to refer to the
contract. 

We will write four main methods of our contract:
- `get_reserves()` - to get values of the reserves deposited at the pool.
- `mint()` - used to deposit liquidity to the pool and mint the LP coins.
- `burn()` - used to withdraw liquidity and burn LP coins.
- `swap()` - to execute a coin swap.

The differences in code that are specific to Fuel will be those that relate
to handling asset transfers as Fuel handles every asset in a native way, details 
will be explained in the implementation section of each method.

Once we have our contract ready we will write some basic harness to test its functionality.
We will use the same [framework](https://github.com/jecikpo/Tutorial-Fuel-SRC20/?tab=readme-ov-file#testing-framework-overview) for writing tests as we did in the SRC20 tutorial.

## Implementation

Let's start the implementation by creating a new Sway project:

```bash
forc new FuniSwapV2
```

Our FuniSwapV2 DEX will have a slightly different directory structure, as we will have 
multiple contracts in this project (this part only covers one, but we need to prepare it 
for the next ones). Each contract will have a subdirectory within the project dir. The 
harness tests directory will be common. Let's create the directory for the pair contract:

```bash
cd FuniSwapV2
mkdir FuniSwapV2Pair
mv src FuniSwapV2Pair
mv FuniSwapV2Pair/src/main.sw FuniSwapV2Pair/src/funi_pair.sw
mv Forc.toml FuniSwapV2Pair/
```
Now let's create the main `Forc.toml` file and we will reference specific per-contract
`Forc.toml` in it.

```bash
cat > Forc.toml << EOF
[workspace]
members = [
    "./SRC20",
    "./FuniSwapV2Pair"
]
EOF
```

As you can see that it references `Forc.toml` files in the `FuniSwapV2Pair` and `SRC20`. Yes, we will
need to have SRC20 contracts deployed to test our DEX. You can just copy the entire `SRC20` dir 
from the Tutorial's [repo](https://github.com/jecikpo/Tutorial-Sway-UniswapV2/tree/main/SRC20).
The SRC20 code there is similar to the one covered in the SRC20 tutorial, hence no point in 
covering it here.

let's also update the `FuniSwapV2Pair/Forc.toml` file to reflect the name of our contract, 
it should look like this:

```conf
[project]
authors = ["JecikPo"]
entry = "funi_pair.sw"
license = "Apache-2.0"
name = "FuniSwapV2Pair"

[dependencies]
```

Now we have our project directory structure ready. When we issue the `forc build` from the main
projects directory it will build both the SRC20 and FuniSwapV2Pair contracts.

### Initial Code

Now we can start building our main pool contract: `funi_swap.sw`. First delete it's contents.

Our Sway source file will be of contract type hence we start with defining it:

```rust
contract;
```

We will add all the necessary imports from the libraries:
```rust
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
```

Now we will define couple of constants. Our pool contract is also an SRC20 contract that mints
The LP tokens, hence we need to define the name, symbol and its decimals:

```rust
const NAME: str[5] = __to_str_array("FuSV2");

const SYMBOL: str[3] = __to_str_array("FV2");

const DECIMALS: u8 = 9;
```

They will be static, as don't need to complicate the code further by allow to customise those.

We will also define the minimum liquidity value just like it is in UniswapV2:

```rust
const MINIMUM_LIQUIDITY: u64 = 1000;
```

We have all necessary constants. Now let's put the `configurable` section, where 
the variables defining our Asset IDs pair that the pool's reserves will be consisting of:

```rust
configurable {
    token0: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
    token1: AssetId = AssetId::from(0x0000000000000000000000000000000000000000000000000000000000000000),
}
```

They are assigned with zero values, but they need to be changed to a meaningful Asset Ids during 
pool deployment. Now let's define our storage layout:

```rust
storage {
    // reserves - deposits turned into liquidity.
    reserve0: u64 = 0,
    reserve1: u64 = 0,

    /// SRC20 ABI
    // The total number of distinguishable assets minted by this contract.
    total_assets: u64 = 1,
    // The total supply of coins for a specific asset minted by this contract.
    total_supply: u64 = 0,
}
```

We define here `reserve0` and `reserve1` which will store the amount of reserve of both assets 
held by the pool. The `total_assets` and `total_supply` are necessary to support the SRC20 standard.

Now let's define our `abi`. We will create two sections to logically separate the SRC20 ABI from the pool ABI.

First, the SRC20 ABI which you should already by aquainted with after the SRC20 tutorial:

```rust
abi SRC20 {
    #[storage(read)]
    fn total_assets() -> u64;

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64>;

    fn name(asset: AssetId) -> Option<String>;

    fn symbol(asset: AssetId) -> Option<String>;

    fn decimals(asset: AssetId) -> Option<u8>;
}
```

Then we define the pool ABI:

```rust
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
```

The only thing left is the implementation of the methods. You can copy the whole `impl SRC20 for Contract`
from the [funi_pair.sw](https://github.com/jecikpo/Tutorial-Sway-UniswapV2/blob/main/FuniSwapV2Pair/src/funi_pair.sw) source code in the repo.

let's prepare the empty `impl` for the pool methods:

```rust
impl FuniSwapV2Pair for Contract {

}
```

Now we are ready for the core functionality of our contract.

### get_reserves Method
The `get_reserves()` method is used to get the amount of reserves of the pool. It will return 
the amount of the reserves stored at the `reserve0` and `reserve1` storage variables.

It needs to go into the `impl FuniSwapV2Pair for Contract` block:
```rust
    #[storage(read)]
    fn get_reserves() -> (u64, u64) {
        _get_reserves()
    }
```

But, we don't have the `_get_reserves()` defined. This is going to be an internal function. Not part
of the ABI, hence we put it outside of the `impl` blocks. Just put it at the end of the source file:
```rust
#[storage(read)]
fn _get_reserves() -> (u64, u64) {
    (
        storage.reserve0.read(),
        storage.reserve1.read()
    )
}
```

Why did we define the internal function like this and make a wrapper in the ABI `impl` block? Because
in Sway unlike in Solidity you are not allowed to call internally the ABI methods and we will need
access to the reserve values from within other methods.

We are done with this method.

### mint Method
`mint()` is used to provide liquidity for the pool and to obtain the LP token amount
that represents accordingly the amount of assets provided. The method works in almost the same way as 
in UniswapV2 original Solidity code:
- it expects that the new reserves are provided beforehand. The amounts should be calculated correctly
so that they don't decrease the `K` value and the LP doesn't lose unnecessarily one of the tokens. 
- it reduces the initial mint by `MINIMUM_AMOUNT` to prevent the infamous "first depositor" issue.
- the LP tokens can be minted (and sent) to a specified address.

Let's create the initial block:
```rust
    #[storage(read, write)]
    fn mint(to: Identity) -> u64 {

    }
```

Now let's add the necessary variables:
```rust
        let total_supply = storage.total_supply.read();
        let mut liquidity = 0;
        let (reserve0, reserve1) = _get_reserves();
        let balance0 = this_balance(token0);
        let balance1 = this_balance(token1);
        let amount0 = balance0 - reserve0;
        let amount1 = balance1 - reserve1;
```
The `total_supply`, `reserve0` and `reserve1` will be needed to calculate `liquidity` (the amount of LP tokens
minted). `amount0` and `amount1` represent the added liquidity. Now let's calculate it:
```rust
        if total_supply == 0 {
            liquidity = (amount0 * amount1).sqrt() - MINIMUM_LIQUIDITY;
            storage.total_supply.write(MINIMUM_LIQUIDITY); // instead of mint 1000 to 0
        } else {
            liquidity = _min(
                (amount0 * total_supply) / reserve0,
                (amount1 * total_supply) / reserve1
            )
        }
```
Just like in Uniswap, the first LP provider gets a haircut equal to `MINIMUM_LIQUIDITY` as explained earlier.
If the LP is not the first, it gets the minimal proportional amount out of two reserves.

Next we need to verify if the `liquidity` is actually greater than zero:
```rust
        require(liquidity > 0, "Insufficient Liquidity");
```

At this point we are sure that we can succesfully mint the LP tokens, update the contract's state and emit
an event:
```rust
        _mint(to, liquidity);
        _update(balance0, balance1, reserve0, reserve1);
        
        log(MintEvent{
            sender: msg_sender().unwrap(),
            to,
            amount0,
            amount1,
        });
```
We didn't define yet the `MintEvent` struct. We will do it at the end of the contract implementation in a 
dedicated section as it will involve library creation. The only thing left in this method is the return
value:
```rust
        liquidity
```

Now, we used couple of internal functions here: `_min()`, `_mint()`, `_update()` which also need to be defined. 
Their contents is quite self explanatory, let's add them at the end of the file:
```rust
#[storage(read, write)]
fn _mint(recipient: Identity, amount: u64) {
    // Increment total supply of the asset and mint to the recipient.
    storage.total_supply.write(amount + storage.total_supply.read());
    mint_to(recipient, DEFAULT_SUB_ID, amount);
}

fn _min(a: u64, b: u64) -> u64 {
    if a >= b {
        a
    } else {
        b
    }
}

#[storage(read, write)]
fn _update(balance0: u64, balance1: u64, reserve0: u64, reserve1: u64) {
    storage.reserve0.write(balance0);
    storage.reserve1.write(balance1);
}
```

That concludes our `mint()` implementation.

### burn Method
`burn()` is used to take back liquidity from the pool back to the user in exchange
for the LP tokens. The method's code is quite similar to UniswapV2:
- it assumes that the LP token is transfered to the contract with the call. This is 
possible only on Fuel as it allows to treat the LP token as Native Asset.
- the liquidity can be transfered back to a specified Identity.

We start by creating an initial block:
```rust
    #[payable]
    #[storage(read, write)]
    fn burn(to: Identity) -> (u64, u64) {

    }
```
The payable attribute is necessry to provide the LP tokens along with the call. 

[NOTE]: You are probably wondering why we couldn't do the same thing with `mint()`
function, instead we just rely on sending the assets *before* the call. The reason 
is that in Fuel you can only send one Native Asset in a call, and for `mint()` we 
need two. Hence we need to create some other mechanism in FuniSwapV2 for obtaining 
the tokens. We will solve that in the later parts where we will implement the Router
contract.

Now let's declare variables inside out method and assign them some values:
```rust
        let total_supply = storage.total_supply.read();
        let liquidity = msg_amount();
        let (reserve0, reserve1) = _get_reserves();
        let mut balance0 = this_balance(token0);
        let mut balance1 = this_balance(token1);
```
We need here the `total_supply` and balances so that we can calculate the amount 
of reserves that are taken out. The amount of LP tokens provided by the caller is taken
from the `msg_amount()`. From the SRC20 tutorial you should remember that the `msg_amount()`
will return the amount for any Asset Id sent with the call. This might not seem to be what
we want. We must verify that the Asset Id sent is the Default Asset Id of this contract.
Don't worry, this will be ensured later when burning tokens, as you cannot really burn 
other tokens than those minted by our contract (remember that to burn tokens in the UTXO
model they need to be first sent to the contract).

Now we can calculate how many tokens are we getting out:
```rust
        let amount0 = (liquidity * balance0) / total_supply;
        let amount1 = (liquidity * balance1) / total_supply;
        require(amount0 > 0 && amount1 > 0, "Insufficient Liquidity Burned");
```

and verify if none of the value is zero. Let's now burn the LP tokens and transfer
the liquidity out:
```rust
        _burn(liquidity);
        transfer(to, token0, amount0);
        transfer(to, token1, amount1);
```

We will define `_burn()` at the end of this section, while `transfer()` is a function
defined in Sway standard library. Now the only thing left is to update the reserves, log 
the event and return the burned amounts.

```rust
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
```

Let's define `_burn()`. This is an internal function hence we put it at the end of the file:

```rust
#[storage(read, write)]
fn _burn(amount: u64) {
    require(
        msg_asset_id() == AssetId::default(),
        "Incorrect asset provided",
    );

    storage.total_supply.write(storage.total_supply.read() - amount);
    burn(DEFAULT_SUB_ID, amount);
}
```
First we require that the provided asset is the default Asset Id of our contract.
This verification is not necessary, but it provides a meaningful message if incorrect
assets are transfered along with the call. Why is that not necessary? Because if the default
Asset Id of `amount` is not sitting at the contract already, the `burn()` function will revert.
We also need to decrease the `total_supply` storage variable and we finally burn the tokens.

We are done with `burn()`, let's move to `swap()`

### swap Method
`swap()` is the core functionality of our pool contract. It's calculations of the "in"
amounts is the same as in UniswapV2. It has the following features:
- it expects that the "in" tokens are already transfered to the contract before calling the 
method. The user specifies the amount "out" and the amount "in" is calculated based on that,
hence the user is expected to transfer the correct amount or the swap will either fail if it 
is lower, or it succeeds, but the excess stays at the pool to the benefit of all LPs.
- the "out" tokens can be transfered to a specified address.
- we will skip for now the callback functionality.

Let's start by preparing the empty method block.

```rust
    #[storage(read, write)]
    fn swap(amount0_out: u64, amount1_out: u64, to: Identity) {

    }
```

We start the implementation of this method by verifying the input correctness:
```rust
        require(
            amount0_out > 0 || amount1_out > 0, 
            "Insufficient Output Amount"
        );
```

Then we get the reserves and we need to check if we are not swapping more than 
the amounts available.
```rust
        let (reserve0, reserve1) = _get_reserves();
        require(
            amount0_out < reserve0 && amount1_out < reserve1, 
            "Insufficient Liquidity"
        );
```

Now let's optimistically transfer the "out" tokens and record new balances:
```rust
        if amount0_out > 0 {
            transfer(to, token0, amount0_out);
        }
        if amount1_out > 0 {
            transfer(to, token1, amount1_out);
        }
        let balance0 = this_balance(token0);
        let balance1 = this_balance(token1);
```

`this_balance()` function returns the amount tokens of a given Asset Id that are held
by this contract. This is a Sway library function.

Next step would be to get the amounts "in" calculated from the recorded balances and the 
stored reserves:
```rust
        let mut amount0_in = 0;
        let mut amount1_in = 0;

        if balance0 > reserve0 - amount0_out {
            amount0_in = balance0 - (reserve0 - amount0_out);
        }
        if balance1 > reserve1 - amount1_out {
            amount1_in = balance1 - (reserve1 - amount1_out);
        }
        require(amount0_in > 0 || amount1_in > 0, "Insufficient Input Amount");
```

We check at the end if the tokens were actually transferred. 

Now we add the code to verify the *K* invariant, based on UniswapV2:
```rust
        let balance0_adjusted = (balance0 * 1000) - (amount0_in * 3);
        let balance1_adjusted = (balance1 * 1000) - (amount1_in * 3);
        require(
            balance0_adjusted * balance1_adjusted >= reserve0 * reserve1 * 1000000,
            "K Invariant Incorrect"
        );
```
You can see that the calculation includes the 0.3% of the swap fee here.

Finally we conclude the implemantion of the `swap()` method by updating reserves 
and logging the event:
```rust
        _update(balance0, balance1, reserve0, reserve1);

        log(SwapEvent{
            sender: msg_sender().unwrap(),
            to,
            amount0_in,
            amount1_in,
            amount0_out,
            amount1_out,
        });
```

We finished our method. Last step is to add the library with the events to the code.

### Events library
The structs defining our events used in three methods above will be implemented in 
a separate file. Create a new file `events.sw` in `FuniSwapV2/FuniSwapV2Pair/src` dir.
This will be a library not a contract hence it will start with a keyword indicating it:

```rust
library;
```

Next we add our three structs:
```rust
pub struct MintEvent {
    pub sender: Identity,
    pub to: Identity,
    pub amount0: u64,
    pub amount1: u64,
}

pub struct BurnEvent {
    pub sender: Identity,
    pub to: Identity,
    pub amount0: u64,
    pub amount1: u64,
}

pub struct SwapEvent {
    pub sender: Identity,
    pub to: Identity,
    pub amount0_in: u64,
    pub amount1_in: u64,
    pub amount0_out: u64,
    pub amount1_out: u64,
}
```

We also need to indicate the access to those structs in our `funi_pair.sw` file

First we add `mod` statement just after the `contract` keyword:

```rust
mod events;
```

And we define the imports:
```rust
use ::events::{
    MintEvent,
    BurnEvent,
    SwapEvent,
};
```

More on how to use libraries can be found [here](https://docs.fuel.network/docs/sway/sway-program-types/libraries/). As this is 
an internal library we don't need to update the `Forc.toml` file.

Now you can test if the code compiles with: 

```bash
forc build
```

issued from the main project directory.

## Summary
This first part of the tutorial concludes the building of the pair contract. 
An attentive reader would notice the difference in sizes of the variables used 
and its effect of the swapping amount range and pool reserve capacity.

We used in our contract only `u64` types of variables which are significantly 
smaller than `uint112` which in UniswapV2 uses to hold reserves accounting information
and smaller than `uint256` which is used within the `K` validation math. This 
means that our contracts can store much less reserves and can reach arithmetic 
overflow even earlier here:

```rust
require(
    balance0_adjusted * balance1_adjusted >= reserve0 * reserve1 * 1000000,
    "K Invariant Incorrect"
);
```

Let's see how bad it is. We will assume that our pool is a stablecoin pool 
hence our `reserve0` and `reserve1` value will be comparable. We need to 
get the maximum value supported by `u64` divide it by 1000000 and sqare root
it, then make the same for Solidity code.

The maximum value of reserves in that case would be around `4_294_967`, that
is not much, comparing to what is possible in Solidity UniswapV2 code, which 
is: 
```
340,282,366,920,938,463,463,374,607,431,768,211,455
```
and that is slightly less than can be held within the `uint112` variable, 
which is:
```
5,192,296,858,534,827,628,530,496,329,220,095,897,600
```

We need to adjust our code as at this point our pair contract won't 
support higher values. Even taking into account the fact that in Fuel
the assets generally support smaller fractional parts (this is reduced
when bridging assets from Ethereum mainnet). We will take care of this problem
in later parts of the tutorial.

