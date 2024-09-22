use crate::utils::setup::*;
use crate::utils::instance::*;

use fuels::{
    prelude::*,
    types::{
        AssetId,
    }
};

#[tokio::test]
async fn test_src20_name() {
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let mut result = token.clone().call_name(
        get_default_asset_id(token.contract_id())
    ).await;
    assert_eq!(
        result, 
        Some(String::from("Token"))
    );

    result = token.clone()
        .call_name_def_asset_id()
        .await;

    assert_eq!(
        result, 
        Some(String::from("Token"))
    );
}

#[tokio::test]
async fn test_src20_name_incorrect_asset_id() {
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let result = token.clone().call_name(
        AssetId::zeroed()
    ).await;
    assert_eq!(
        result, 
        None
    );
}

#[tokio::test]
async fn test_src20_symbol() {
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let mut result = token.clone().call_symbol(
        get_default_asset_id(token.contract_id())
    ).await;
    assert_eq!(
        result, 
        Some(String::from("TKN"))
    );

    result = token.clone()
        .call_symbol_def_asset_id()
        .await;

    assert_eq!(
        result, 
        Some(String::from("TKN"))
    );
}

#[tokio::test]
async fn test_src20_mint() {
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let amount = 1000;

    let balance_before = token.clone()
        .deployer_balance(
            get_default_asset_id(
                token.contract_id()
            )
        ).await;

    token.clone().call_mint(
        token.clone().deployer_identity(),
        DEFAULT_SUB_ID,
        amount
    ).await;

    let balance_after = token.clone()
        .deployer_balance(
            get_default_asset_id(
                token.contract_id()
            )
        ).await;

    assert_eq!(
        balance_before + amount, 
        balance_after
    )
}

#[tokio::test]
async fn test_src20_burn() {
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let amount = 1000;

    token.clone().call_mint(
        token.clone().deployer_identity(),
        DEFAULT_SUB_ID,
        amount
    ).await;

    let balance_before = token.clone()
        .deployer_balance(
            get_default_asset_id(
                token.contract_id()
            )
        ).await;

    token.clone().call_burn(
        DEFAULT_SUB_ID,
        amount
    ).await;

    let balance_after = token.clone()
        .deployer_balance(
            get_default_asset_id(
                token.contract_id()
            )
        ).await;
    
    assert_eq!(
        balance_before, 
        balance_after + amount
    );
}

#[tokio::test]
async fn test_src20_total_supply() {
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new().await;
    let amount = 1000;

    let total_supply_before = token.clone()
        .call_total_supply_def_asset_id()
        .await;

    token.clone().call_mint(
        token.clone().deployer_identity(),
        DEFAULT_SUB_ID,
        amount
    ).await;

    let total_supply_after = token.clone()
        .call_total_supply_def_asset_id()
        .await;

    assert_eq!(
        total_supply_before + amount,
        total_supply_after
    )
}

#[tokio::test]
async fn test_src20_configurables() {
    let name = "SRC20";
    let symbol = "S20";
    let decimals = 10;
    let configurables = create_src20_configurables(name, symbol, decimals);
    let token = ContractInstance::<SRC20<WalletUnlocked>>::new_with_configurables(configurables).await;

    let result_symbol = token.clone().call_symbol(
        get_default_asset_id(token.contract_id())
    ).await;
    assert_eq!(
        result_symbol, 
        Some(String::from(symbol))
    );

    let result_name = token.clone().call_name(
        get_default_asset_id(token.contract_id())
    ).await;

    assert_eq!(
        result_name, 
        Some(String::from(name))
    );

    let result_decimals = token.clone().call_decimals(
        get_default_asset_id(token.contract_id())
    ).await;

    assert_eq!(
        result_decimals, 
        Some(decimals as u8)
    );
}

