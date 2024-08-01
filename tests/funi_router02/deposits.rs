use crate::utils::setup::*;
use crate::utils::funi_router02::*;
use crate::utils::src20::*;
use crate::utils::instance::*;

use fuels::{
    prelude::*,
    types::{
        AssetId,
    }
};

#[tokio::test]
async fn test_funi_deposit() {
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let funi_router02_configurables = create_funi_router02_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let router02 = ContractInstance::<FuniSwapV2Router02<WalletUnlocked>>::new_with_configurables(funi_router02_configurables).await;
    let depositor = router02.clone().deployer_identity();
    let amount0 = 1000;
    let amount1 = 10000;

    /* mint some token0 and token1 for the depositor */
    token0.clone().call_mint(depositor, DEFAULT_SUB_ID, amount0).await;
    token1.clone().call_mint(depositor, DEFAULT_SUB_ID, amount1).await;

    /* deposit tokens into the router02 contract */
    router02.clone().call_deposit(
        depositor,
        token0_asset_id,
        amount0
    ).await;
    router02.clone().call_deposit(
        depositor,
        token1_asset_id,
        amount1
    ).await;

    /* get the deposited amounts */
    let (deposit0, deposit1) = router02.call_get_deposits(depositor).await;

    assert_eq!(deposit0, amount0);
    assert_eq!(deposit1, amount1);
}

#[tokio::test]
async fn test_funi_withdraw() {
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let funi_router02_configurables = create_funi_router02_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let router02 = ContractInstance::<FuniSwapV2Router02<WalletUnlocked>>::new_with_configurables(funi_router02_configurables).await;
    let depositor = router02.clone().deployer_identity();
    let amount0 = 1000;
    let amount1 = 10000;

    /* mint some token0 and token1 for the depositor */
    token0.clone().call_mint(depositor, DEFAULT_SUB_ID, amount0).await;
    token1.clone().call_mint(depositor, DEFAULT_SUB_ID, amount1).await;

    /* record balance after minting and before deposit */
    let token0_balance_before = router02.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_before = router02.clone().deployer_balance(token1_asset_id).await;

     /* deposit tokens into the router02 contract */
     router02.clone().call_deposit(
        depositor,
        token0_asset_id,
        amount0
    ).await;
    router02.clone().call_deposit(
        depositor,
        token1_asset_id,
        amount1
    ).await;

    /* withdraw what was deposited */
    router02.clone().call_withdraw(depositor).await;

    /* record balance after withdraw */
    let token0_balance_after = router02.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_after = router02.clone().deployer_balance(token1_asset_id).await;

    assert_eq!(token0_balance_before, token0_balance_after);
    assert_eq!(token1_balance_before, token1_balance_after);
}

#[tokio::test]
async fn test_funi_contract_deposit_balance() {
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let funi_router02_configurables = create_funi_router02_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let router02 = ContractInstance::<FuniSwapV2Router02<WalletUnlocked>>::new_with_configurables(funi_router02_configurables).await;
    let depositor = get_deployer_identity().await;
    let amount0 = 1000;
    let amount1 = 10000;

    /* mint some token0 and token1 for the depositor */
    token0.clone().call_mint(depositor, DEFAULT_SUB_ID, amount0).await;
    token1.clone().call_mint(depositor, DEFAULT_SUB_ID, amount1).await;

    /* record balance after minting and before deposit */
    let token0_balance_before = router02.clone().deployer_balance(token0_asset_id).await;
    let token1_balance_before = router02.clone().deployer_balance(token1_asset_id).await;

     /* deposit tokens into the router02 contract */
     router02.clone().call_deposit(
        depositor,
        token0_asset_id,
        amount0
    ).await;
    router02.clone().call_deposit(
        depositor,
        token1_asset_id,
        amount1
    ).await;

    let router02_token0_balance = router02.get_contract_balance(token0_asset_id).await;
    let router02_token1_balance = router02.get_contract_balance(token1_asset_id).await;

    assert_eq!(amount0, router02_token0_balance);
    assert_eq!(amount1, router02_token1_balance);
}

#[tokio::test]
async fn test_funi_contract_withdraw_balance() {
    let token0 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token1 = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let token0_asset_id = token0.clone().get_default_asset_id();
    let token1_asset_id = token1.clone().get_default_asset_id();
    let funi_router02_configurables = create_funi_router02_configurables(
        token0_asset_id,
        token1_asset_id
    );
    
    let router02 = ContractInstance::<FuniSwapV2Router02<WalletUnlocked>>::new_with_configurables(funi_router02_configurables).await;
    let depositor = get_deployer_identity().await;
    let amount0 = 1000;
    let amount1 = 10000;

    /* mint some token0 and token1 for the depositor */
    token0.clone().call_mint(depositor, DEFAULT_SUB_ID, amount0).await;
    token1.clone().call_mint(depositor, DEFAULT_SUB_ID, amount1).await;

     /* deposit tokens into the router02 contract */
     router02.clone().call_deposit(
        depositor,
        token0_asset_id,
        amount0
    ).await;
    router02.clone().call_deposit(
        depositor,
        token1_asset_id,
        amount1
    ).await;

    /* record balance after minting and depositing */
    let token0_balance_before = router02.get_contract_balance(token0_asset_id).await;
    let token1_balance_before = router02.get_contract_balance(token1_asset_id).await;

    /* withdraw what was deposited */
    router02.clone().call_withdraw(depositor).await;

    /* record balance after withdrawing */
    let token0_balance_after = router02.get_contract_balance(token0_asset_id).await;
    let token1_balance_after = router02.get_contract_balance(token1_asset_id).await;

    assert_eq!(token0_balance_before, token0_balance_after + amount0);
    assert_eq!(token1_balance_before, token1_balance_after + amount1);
}
