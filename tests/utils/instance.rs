use fuels::{
    prelude::*, 
    types::ContractId, 
    types::{
        AssetId,
        Bits256,
        Identity,
    }
};

use crate::utils::setup::{
    get_default_asset_id,
    DEFAULT_GAS_LIMIT,
    DEFAULT_SUB_ID,
};

#[derive(Clone)]
pub struct ContractInstance<T> {
    pub instance: T,
    pub contract_id: ContractId,
    pub wallet: WalletUnlocked,
    pub gas_limit: u64,
}

pub trait GenericMethods {
    // return current instance contract ID
    fn contract_id(&self) -> ContractId;

    // return the identity based on the wallet
    fn deployer_identity(&self) -> Identity;

    fn deployer_wallet(&self) -> WalletUnlocked;

    // return the balance of wallet which was used to deploy the instance
    async fn deployer_balance(&self, asset_id: AssetId) -> u64;

    // get the default AssetID of the deployed contract
    fn get_default_asset_id(&self) -> AssetId;
}

impl<T> GenericMethods for ContractInstance<T> 
where
    T: Clone
{
    fn contract_id(&self) -> ContractId {
        self.contract_id
    }

    fn deployer_identity(&self) -> Identity {
        Identity::Address(
            Address::from(
                self.wallet.address()
            )
        )
    }

    fn deployer_wallet(&self) -> WalletUnlocked {
        self.wallet.clone()
    }

    async fn deployer_balance(&self, asset_id: AssetId) -> u64 {
        self.wallet.get_asset_balance(&asset_id).await.unwrap()
    }
    
    fn get_default_asset_id(&self) -> AssetId {
        get_default_asset_id(self.contract_id)
    }
}

pub trait WalletTransfer {
    async fn transfer_to_contract(&self, contract: ContractId, asset: AssetId, amount: u64);
}

impl WalletTransfer for WalletUnlocked {
    async fn transfer_to_contract(&self, contract: ContractId, asset: AssetId, amount: u64) {
        let bech32_contract_id = Bech32ContractId::from(contract);
        let _ = self.force_transfer_to_contract(
            &bech32_contract_id,
            amount,
            asset,
            TxPolicies::default().with_script_gas_limit(400000).with_max_fee(400000)
        ).await;
    }
}