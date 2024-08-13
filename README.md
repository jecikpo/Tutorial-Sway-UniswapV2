# Tutorial Sway UniswapV2 (FuniSwapV2)
This tutorial explains implementation of the famous UniswapV2 DEX in Sway. We will 
see that there is a couple of differences comparing to the original Solidity code
that we have to redesign in order to make it fit for the Fuel VM.

Prerequisites:
1) Knowledge of the Fuel VM and Sway language. You can start with my previous tutorial on [SRC20](https://github.com/jecikpo/Tutorial-Fuel-SRC20).
2) Familiarity with UniswapV2 code. Great resource can be found in the [UniswapV2 Book](https://www.rareskills.io/uniswap-v2-book).

The tutorial is split into parts:
1) [Part 1 - Pair](https://github.com/jecikpo/Tutorial-Sway-UniswapV2/blob/main/PART-1-Pair.md) - the equivalent of UniswapV2 UniswapV2Pair.sol - where the core of the LP and swapping logic resides.
2) Part 2 - Pair Tests - Tests of the pair contract.
2) Part 3 - Factory - this is where we define pool creations through the Factory (originally the UniswapV2Factory).
3) Part 4 - Router - Here we define the periphery contract for interacting safely with different pools. 

We call this implementation *FuniSwapV2* because it is a Fuel version of UniswapV2.

## Running

You can download and launch the code according to the following instructions:

```bash
git clone https://github.com/jecikpo/Tutorial-Sway-UniswapV2
```

Then build the project:

```bash
cd Tutorial-Sway-UniswapV2
forc build
```

and run tests:
```bash
cargo test
```