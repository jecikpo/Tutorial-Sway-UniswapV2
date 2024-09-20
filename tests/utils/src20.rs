
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
    SRC20,
    SRC20Configurables,
    get_src20_contract_instance,
    get_src20_contract_instance_with_configurables,
    get_default_asset_id,
    DEFAULT_GAS_LIMIT,
    DEFAULT_SUB_ID,
};


use crate::utils::instance::{
    ContractInstance,
};

impl ContractInstance<SRC20<WalletUnlocked>> {
    /*
      Constructor of the SRC20 token
     */
    pub async fn new() -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_src20_contract_instance().await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }

    pub async fn new_with_configurables(configurables: SRC20Configurables) -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_src20_contract_instance_with_configurables(configurables).await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }

    /*
      Call name() function and return result
     */
    pub async fn call_name(self, asset_id: AssetId) -> Option<String> {
        self.instance.clone()
            .with_account(self.wallet.clone())
            .methods()
            .name(asset_id) // smart contract function
            .with_tx_policies(
                TxPolicies::default()
                .with_script_gas_limit(self.gas_limit)
            )
            .call()
            .await
            .unwrap()
            .value
    }

    /*
      Get the Contract Id of the deployed contract.
     */
    pub fn contract_id(&self) -> ContractId {
        self.contract_id
    }

    /*
      Call name() function with default Asset ID and return result
     */
    pub async fn call_name_def_asset_id(self) -> Option<String> {
        self.clone().call_name(
            get_default_asset_id(self.contract_id)
        ).await
    }

    /*
      Call symbol() function and return result
     */
    pub async fn call_symbol(self, asset_id: AssetId) -> Option<String> {
        self.instance.clone()
        .with_account(self.wallet.clone())
        .methods()
        .symbol(asset_id) // smart contract function
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call()
        .await
        .unwrap()
        .value
    }

    /*
      Call symbol() function with default Asset ID and return result
     */
    pub async fn call_symbol_def_asset_id(self) -> Option<String> {
        self.clone().call_symbol(
            get_default_asset_id(self.contract_id)
        ).await
    }

    /*
      Call decimals() function and return result
     */
     pub async fn call_decimals(self, asset_id: AssetId) -> Option<u8> {
        self.instance.clone()
        .with_account(self.wallet.clone())
        .methods()
        .decimals(asset_id) // smart contract function
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call()
        .await
        .unwrap()
        .value
    }

    /*
      Call symbol() function with default Asset ID and return result
     */
    pub async fn call_decimals_def_asset_id(self) -> Option<u8> {
        self.clone().call_decimals(
            get_default_asset_id(self.contract_id)
        ).await
    }

    /* 
        Get the Identity of the deployer.
     */
    pub fn deployer_identity(self) -> Identity {
        Identity::Address(
            Address::from(
                self.wallet.address()
            )
        )
    }

    pub async fn deployer_balance(self, asset_id: AssetId) -> u64 {
        self.wallet.get_asset_balance(&asset_id).await.unwrap()
    }

    /*
      call mint() function.
     */
    pub async fn call_mint(self, recipient: Identity, sub_id: Bits256, amount: u64) {
        self.instance.clone()
        .with_account(self.wallet)
        .methods()
        .mint(recipient, sub_id, amount)
        .with_variable_output_policy(VariableOutputPolicy::Exactly(2))
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call()
        .await
        .unwrap();
    }

    pub async fn call_mint_default(self, recipient: Identity, amount: u64) {
        self.call_mint(recipient, DEFAULT_SUB_ID, amount);
    }

    /*
      call mint() function.
     */
    pub async fn call_burn(self, sub_id: Bits256, amount: u64) {
        self.instance.clone()
        .with_account(self.wallet)                   // <- called by
        .methods()
        .burn(sub_id, amount)                        // <- actual burn()
//        .append_variable_outputs(2)                  // <- append outputs in case UTXO is left
        .with_tx_policies(                           // setup gas so that it doesn't revert
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call_params(CallParameters::new(            // transfer coins so that they can be burn
            amount,
            get_default_asset_id(self.contract_id),
            self.gas_limit,
        )).unwrap()
        .call()
        .await
        .unwrap();                                   // unwrap u64
    }

    /*
        call total_supply() 
     */
    pub async fn call_total_supply(self, asset_id: AssetId) -> Option<u64> {
        self.instance.clone()
            .with_account(self.wallet)
            .methods()
            .total_supply(asset_id)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(2))
            .with_tx_policies(
                TxPolicies::default()
                .with_script_gas_limit(self.gas_limit)
            )
            .call()
            .await
            .unwrap()
            .value
    }

    /*
        call total_supply() with default asset id.
        No need to check for output because always a u64 will be returned
     */
    pub async fn call_total_supply_def_asset_id(self) -> u64 {
        self.clone().call_total_supply(
            get_default_asset_id(self.contract_id)
        ).await
        .unwrap()
    }

    /*
        Return default AssetId of the current contract
     */
    pub fn get_default_asset_id(self) -> AssetId {
        get_default_asset_id(self.contract_id)
    }
}