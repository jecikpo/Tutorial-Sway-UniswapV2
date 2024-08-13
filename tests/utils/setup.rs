use fuels::{
    prelude::*, 
    types::ContractId, 
    crypto::SecretKey, 
    types::{
        AssetId,
        Bytes32,
        Bits256,
        SizedAsciiString,
        Identity,
    }
};

// Load abi from json
abigen!(
    Contract(
        name = "SRC20",
        abi = "./SRC20/out/debug/src20-abi.json"
    ),
    Contract(
        name = "FuniSwapV2Pair",
        abi = "./FuniSwapV2Pair/out/debug/FuniSwapV2Pair-abi.json"
    ),
    Contract(
        name = "FuniSwapV2Router02",
        abi = "./FuniSwapV2Router02/out/debug/FuniSwapV2Router02-abi.json"
    )
);


use rand::Rng;
use std::str::FromStr;
use sha2::{Digest, Sha256};

pub const DEFAULT_GAS_LIMIT: u64 = 400000;
pub const DEFAULT_SUB_ID: Bits256 = Bits256([0; 32]);

pub const SECRECT_KEY: &str = "<YOUR_SECRECT_KEY_HERE>";

pub const FUEL_NETWORK: &str = "127.0.0.1:4000";
//pub const FUEL_NETWORK: &str = "testnet.fuel.network";

/*
 * ---- Generic Wallet creation
 */

pub async fn get_wallet_provider_salt() -> (Provider, WalletUnlocked, Salt) {
    // Launch a local network and deploy the contract
    let provider = Provider::connect(FUEL_NETWORK).await.unwrap();

    let secret = match SecretKey::from_str(
        SECRECT_KEY
    ) {
        Ok(value) => value,
        Err(e) => panic!("unable to create secret: {}", e),
    };

    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));

    // Generate a random 32-byte array
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);

    let salt = Salt::new(bytes);

    (provider, wallet, salt)
}

/*
 * ---- SRC20 setup functions
 */

pub async fn get_src20_contract_instance() -> (SRC20<WalletUnlocked>, ContractId, WalletUnlocked, AssetId) {
    
    let (provider, wallet, salt) = get_wallet_provider_salt().await;

    let id = Contract::load_from(
        "./SRC20/out/debug/src20.bin",
        LoadConfiguration::default().with_salt(salt),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000))
    .await
    .unwrap();

    let instance = SRC20::new(id.clone(), wallet.clone());
    let base_asset_id = provider.base_asset_id();

    (instance, id.into(), wallet, *base_asset_id)
}

pub async fn get_src20_contract_instance_with_configurables(configurables: SRC20Configurables) -> (SRC20<WalletUnlocked>, ContractId, WalletUnlocked, AssetId) {
    
    let (provider, wallet, salt) = get_wallet_provider_salt().await;

    let id = Contract::load_from(
        "./SRC20/out/debug/src20.bin",
        LoadConfiguration::default()
        .with_salt(salt)
        .with_configurables(configurables),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000))
    .await
    .unwrap();

    let instance = SRC20::new(id.clone(), wallet.clone());
    let base_asset_id = provider.base_asset_id();

    (instance, id.into(), wallet, *base_asset_id)
}

pub fn create_src20_configurables(name: &str, symbol: &str, decimals: u8) -> SRC20Configurables {
    let name_configurable: SizedAsciiString<5> = name.try_into().unwrap();
    let symbol_configurable: SizedAsciiString<3> = symbol.try_into().unwrap();

    SRC20Configurables::default()
    .with_name(name_configurable).unwrap()
    .with_symbol(symbol_configurable).unwrap()
    .with_decimals(decimals).unwrap()
}

/*
 * ---- FuniSwapV2Pair Setup Functions
 */

pub async fn get_funi_pair_contract_instance() -> (FuniSwapV2Pair<WalletUnlocked>, ContractId, WalletUnlocked, AssetId) {
    
    let (provider, wallet, salt) = get_wallet_provider_salt().await;

    let id = Contract::load_from(
        "./FuniSwapV2Pair/out/debug/FuniSwapV2Pair.bin",
        LoadConfiguration::default().with_salt(salt),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000))
    .await
    .unwrap();

    let instance = FuniSwapV2Pair::new(id.clone(), wallet.clone());
    let base_asset_id = provider.base_asset_id();

    (instance, id.into(), wallet, *base_asset_id)
}

pub async fn get_funi_pair_contract_instance_with_configurables(configurables: FuniSwapV2PairConfigurables) -> (
    FuniSwapV2Pair<WalletUnlocked>, 
    ContractId, 
    WalletUnlocked, 
    AssetId
) {
    let (provider, wallet, salt) = get_wallet_provider_salt().await;

    let id = Contract::load_from(
        "./FuniSwapV2Pair/out/debug/FuniSwapV2Pair.bin",
        LoadConfiguration::default()
        .with_salt(salt)
        .with_configurables(configurables),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000))
    .await
    .unwrap();

    let instance = FuniSwapV2Pair::new(id.clone(), wallet.clone());
    let base_asset_id = provider.base_asset_id();

    (instance, id.into(), wallet, *base_asset_id)
}

pub fn create_funi_pair_configurables(token0: AssetId, token1: AssetId) -> FuniSwapV2PairConfigurables {
    FuniSwapV2PairConfigurables::default()
    .with_token0(token0).unwrap()
    .with_token1(token1).unwrap()
}

/*
 * ---- FuniSwapV2Router02 Setup Functions
 */

pub async fn get_funi_router02_contract_instance() -> (FuniSwapV2Router02<WalletUnlocked>, ContractId, WalletUnlocked, AssetId) {
    
    let (provider, wallet, salt) = get_wallet_provider_salt().await;

    let id = Contract::load_from(
        "./FuniSwapV2Router02/out/debug/FuniSwapV2Router02.bin",
        LoadConfiguration::default().with_salt(salt),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000))
    .await
    .unwrap();

    let instance = FuniSwapV2Router02::new(id.clone(), wallet.clone());
    let base_asset_id = provider.base_asset_id();

    (instance, id.into(), wallet, *base_asset_id)
}

pub async fn get_funi_router02_contract_instance_with_configurables(configurables: FuniSwapV2Router02Configurables) -> (
    FuniSwapV2Router02<WalletUnlocked>, 
    ContractId, 
    WalletUnlocked, 
    AssetId
) {
    let (provider, wallet, salt) = get_wallet_provider_salt().await;

    let id = Contract::load_from(
        "./FuniSwapV2Router02/out/debug/FuniSwapV2Router02.bin",
        LoadConfiguration::default()
        .with_salt(salt)
        .with_configurables(configurables),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000))
    .await
    .unwrap();

    let instance = FuniSwapV2Router02::new(id.clone(), wallet.clone());
    let base_asset_id = provider.base_asset_id();

    (instance, id.into(), wallet, *base_asset_id)
}

pub fn create_funi_router02_configurables(token0: AssetId, token1: AssetId) -> FuniSwapV2Router02Configurables {
    FuniSwapV2Router02Configurables::default()
    .with_token0(token0).unwrap()
    .with_token1(token1).unwrap()
}

/*
 * ---- Generic Utils
 */

pub fn get_asset_id(sub_id: Bytes32, contract: ContractId) -> AssetId {
    let mut hasher = Sha256::new();
    hasher.update(*contract);
    hasher.update(*sub_id);
    AssetId::new(*Bytes32::from(<[u8; 32]>::from(hasher.finalize())))
}

pub fn get_default_asset_id(contract: ContractId) -> AssetId {
    let default_sub_id = Bytes32::from([0u8; 32]);
    get_asset_id(default_sub_id, contract)
}

pub async fn get_deployer_identity() -> Identity {
    let (_provider, wallet, _salt) = get_wallet_provider_salt().await;
    Identity::Address(
        Address::from(
            wallet.address()
        )
    )
}


