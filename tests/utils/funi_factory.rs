use fuels::{
    prelude::*, 
    types::ContractId, 
    types::{
        AssetId,
        Identity,
    }
};

use crate::utils::setup::{
    FuniSwapV2Factory,
    get_funi_factory_contract_instance,
    get_default_asset_id,
    DEFAULT_GAS_LIMIT,
};

use crate::utils::instance::{
    ContractInstance,
};

impl ContractInstance<FuniSwapV2Factory<WalletUnlocked>> {
    /*
      Constructor of the SRC20 token
     */
    pub async fn new() -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_funi_factory_contract_instance().await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }

    pub fn get_instance(&self) -> FuniSwapV2Factory<WalletUnlocked> {
        self.instance.clone()
    }

    /*
      call initialize() function.
     */
     pub async fn call_initialize(&mut self, pair: ContractId) {
        let bech32_pair_contract_id = Bech32ContractId::from(pair);

        let _ = self.instance.clone()
        .with_account(self.wallet.clone())
        .methods()
        .initialize(pair)
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .with_contract_ids(&[bech32_pair_contract_id])
        .call()
        .await
        .unwrap();
    }

    /*
      call initialize() function.
     */
     pub async fn call_create_pair(&mut self, token0: AssetId, token1: AssetId, pair: ContractId) {
        let bech32_pair_contract_id = Bech32ContractId::from(pair);

        let _ = self.instance.clone()
        .with_account(self.wallet.clone())
        .methods()
        .create_pair(token0, token1, pair)
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .with_contract_ids(&[bech32_pair_contract_id])
        .call()
        .await
        .unwrap();
    }

}