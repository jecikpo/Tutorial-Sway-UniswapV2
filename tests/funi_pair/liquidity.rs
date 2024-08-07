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

#[tokio::test]
async fn test_funi_mint_initial() {
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let funi_pair_configurables = create_funi_pair_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let pair = ContractInstance::<FuniSwapV2Pair<WalletUnlocked>>::new_with_configurables(funi_pair_configurables).await;
    let depositor = get_deployer_identity().await;
    let amount0 = 1000;
    let amount1 = 10000;

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

    let expected_liquidity = 2162;
    /* deploy initial liquidity */
    let liquidity = pair.clone().call_mint(depositor).await;
    assert_eq!(expected_liquidity, liquidity);

    /* get LP token balance */
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
}

#[tokio::test]
async fn test_funi_burn_initial() {
    let amount0 = 1000;
    let amount1 = 10000;
    let expected_amount0_out = 683;
    let expected_amount1_out = 6837;

    /* create pair instance with liquidity deployed */
    let (token0, token1, pair, liquidity) = setup_funi_pair(
        amount0,
        amount1,
    ).await;

    /* record balance before burning */
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let token0_balance_before = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_before = pair.clone().deployer_balance(token1_asset_id).await;

    /* call burn */
    let (received_token0, received_token1) = pair.clone().call_burn(
        pair.deployer_identity(),
        liquidity,
    ).await;

    /* record balance after burning */
    let token0_balance_after = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_after = pair.clone().deployer_balance(token1_asset_id).await;

    /* verify account balance */
    assert_eq!(expected_amount0_out, token0_balance_after - token0_balance_before);
    assert_eq!(expected_amount1_out, token1_balance_after - token1_balance_before);

    /* verify returned values */
    assert_eq!(expected_amount0_out, received_token0);
    assert_eq!(expected_amount1_out, received_token1);
}

#[tokio::test]
async fn test_funi_mint_second() {
    let first_amount0 = 1000;
    let first_amount1 = 10000;
    let second_amount0 = 10000;
    let second_amount1 = 100000;
    let expected_amount0_out = 683;
    let expected_amount1_out = 6837;

    /* create pair instance with liquidity deployed */
    let (token0, token1, pair, liquidity) = setup_funi_pair(
        first_amount0,
        first_amount1,
    ).await;

    let second_liquidity = mint_and_deploy_liquidity(
        token0.clone(),
        token1.clone(),
        pair.clone(),
        second_amount0,
        second_amount1,
    ).await;

    println!("Second liquidity: {}", second_liquidity);

    /* record balance before burning */
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let token0_balance_before = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_before = pair.clone().deployer_balance(token1_asset_id).await;

    /* call burn */
    let (received_token0, received_token1) = pair.clone().call_burn(
        pair.deployer_identity(),
        second_liquidity,
    ).await;

    /* record balance after burning */
    let token0_balance_after = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_after = pair.clone().deployer_balance(token1_asset_id).await;

    /* verify account balance */
    assert_eq!(second_amount0, token0_balance_after - token0_balance_before);
    assert_eq!(second_amount1, token1_balance_after - token1_balance_before);

    /* verify returned values */
    assert_eq!(second_amount0, received_token0);
    assert_eq!(second_amount1, received_token1);
}

#[tokio::test]
async fn test_funi_get_reserves() {
    let amount0 = 1000;
    let amount1 = 10000;

    /* create pair instance with liquidity deployed */
    let (token0, token1, pair, liquidity) = setup_funi_pair(
        amount0,
        amount1,
    ).await;

    let (reserve0, reserve1) = pair.call_get_reserves().await;

    assert_eq!(amount0, reserve0);
    assert_eq!(amount1, reserve1);

}

#[tokio::test]
async fn test_funi_swap() {
    let amount0 = 10000;
    let amount1 = 100000;
    let amount0_in = 1000;
    let amount1_in = 1;
    let amount0_out = 0;
    let amount1_out = 9066; /* calculated amount for 1000 of token 0 */

    /* create pair instance with liquidity deployed */
    let (token0, token1, pair, liquidity) = setup_funi_pair(
        amount0,
        amount1,
    ).await;

    /* mint some tokens for swapping */
    let swapper = get_deployer_identity().await;
    mint_coins(swapper, token0.clone(), token1.clone(), amount0_in, amount1_in).await;

    /* record balance before swapping */
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let token0_balance_before = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_before = pair.clone().deployer_balance(token1_asset_id).await;

    /* transfer assets 0 to pair contract */
    let wallet = pair.deployer_wallet();
    wallet.transfer_to_contract(
        pair.contract_id(),
        token0_asset_id,
        amount0_in
    ).await;

    /* execute the swap */
    pair.clone().call_swap(amount0_out, amount1_out, swapper).await;

    /* record balance after swapping */
    let token0_balance_after = pair.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_after = pair.clone().deployer_balance(token1_asset_id).await;

    assert_eq!(token0_balance_before, token0_balance_after + amount0_in);
    assert_eq!(token1_balance_before, token1_balance_after - amount1_out);
}

/*
    ---- Helper functions
*/

async fn setup_funi_pair(liquidity0: u64, liquidity1: u64) -> (
    ContractInstance::<SRC20<WalletUnlocked>>,
    ContractInstance::<SRC20<WalletUnlocked>>,
    ContractInstance::<FuniSwapV2Pair<WalletUnlocked>>,
    u64
) {
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let funi_pair_configurables = create_funi_pair_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let pair = ContractInstance::<FuniSwapV2Pair<WalletUnlocked>>::new_with_configurables(funi_pair_configurables).await;
    let depositor = get_deployer_identity().await;

    /* mint some token0 and token1 for the depositor */
    token0.clone().call_mint(depositor, DEFAULT_SUB_ID, liquidity0).await;
    token1.clone().call_mint(depositor, DEFAULT_SUB_ID, liquidity1).await;

    /* transfer assets to pair contract */
    let wallet = pair.deployer_wallet();
    wallet.transfer_to_contract(
        pair.contract_id(),
        token0_asset_id,
        liquidity0
    ).await;
    wallet.transfer_to_contract(
        pair.contract_id(),
        token1_asset_id,
        liquidity1
    ).await;

    /* deploy initial liquidity */
    let liquidity = pair.clone().call_mint(depositor).await;
    (token0, token1, pair, liquidity)
}

/*
    Mint tokens and deploy them as liquidity to the given pair
*/
async fn mint_and_deploy_liquidity(
    token0: ContractInstance::<SRC20<WalletUnlocked>>,
    token1: ContractInstance::<SRC20<WalletUnlocked>>,
    pair: ContractInstance::<FuniSwapV2Pair<WalletUnlocked>>,
    amount0: u64,
    amount1: u64,
) -> u64 {
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();

    let depositor = get_deployer_identity().await;

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

    pair.clone().call_mint(depositor).await
}

/*
    Mint some tokens, but don't deploy liquidity. (for swapping)
*/
async fn mint_coins(
    to: Identity,
    token0: ContractInstance::<SRC20<WalletUnlocked>>,
    token1: ContractInstance::<SRC20<WalletUnlocked>>,
    amount0: u64,
    amount1: u64,
) {
    /* mint some token0 and token1 for the to address */
    token0.clone().call_mint(to, DEFAULT_SUB_ID, amount0).await;
    token1.clone().call_mint(to, DEFAULT_SUB_ID, amount1).await;
}