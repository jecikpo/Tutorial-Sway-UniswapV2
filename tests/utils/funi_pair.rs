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
    FuniSwapV2Pair,
    FuniSwapV2PairConfigurables,
    get_funi_pair_contract_instance,
    get_funi_pair_contract_instance_with_configurables,
    get_default_asset_id,
    DEFAULT_GAS_LIMIT,
    DEFAULT_SUB_ID,
};

use crate::utils::instance::{
    ContractInstance,
};

impl ContractInstance<FuniSwapV2Pair<WalletUnlocked>> {
    /*
      Constructor of the SRC20 token
     */
    pub async fn new() -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_funi_pair_contract_instance().await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }

    pub async fn new_with_configurables(configurables: FuniSwapV2PairConfigurables) -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_funi_pair_contract_instance_with_configurables(configurables).await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }


    pub async fn get_contract_balance(&self, asset_id: AssetId) -> u64 {
        let assets = self.instance.clone().get_balances().await.unwrap();
        if let Some(amount) = assets.get(&asset_id) {
            *amount
        } else {
            0
        }
    }

    pub fn get_instance(&self) -> FuniSwapV2Pair<WalletUnlocked> {
        self.instance.clone()
    }

    /*
      call mint() function.
     */
     pub async fn call_mint(self, to: Identity) -> (u64) {
        let result = self.instance.clone()
        .with_account(self.wallet)
        .methods()
        .mint(to)
        .append_variable_outputs(1)
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call()
        .await
        .unwrap();

        result.value
    }

    /*
      call mint() function.
     */
     pub async fn call_burn(self, to: Identity, amount: u64) -> (u64, u64) {
        let result = self.instance.clone()
        .with_account(self.wallet)
        .methods()
        .burn(to)
        .append_variable_outputs(2)
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call_params(CallParameters::new(
            amount,
            get_default_asset_id(self.contract_id),
            self.gas_limit,
        )).unwrap()
        .call()
        .await
        .unwrap();

        result.value
    }
}

