use fuels::{
    prelude::*, 
    types::ContractId, 
    types::{
        AssetId,
        Identity,
    }
};

use crate::utils::setup::{
    FuniSwapV2Router02,
    FuniSwapV2Router02Configurables,
    get_funi_router02_contract_instance,
    get_funi_router02_contract_instance_with_configurables,
    DEFAULT_GAS_LIMIT,
};

use crate::utils::instance::{
    ContractInstance,
};

impl ContractInstance<FuniSwapV2Router02<WalletUnlocked>> {
    /*
      Constructor of the SRC20 token
     */
    pub async fn new() -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_funi_router02_contract_instance().await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }

    pub async fn new_with_configurables(configurables: FuniSwapV2Router02Configurables) -> Self {
        let (instance, contract_id, wallet, _base_asset_id) = get_funi_router02_contract_instance_with_configurables(configurables).await;
        Self {
            instance,
            contract_id,
            wallet,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }

    /*
      call deposit() function.
     */
    pub async fn call_deposit(self, to: Identity, asset: AssetId, amount: u64) {
        self.instance.clone()
        .with_account(self.wallet)
        .methods()
        .deposit(to)
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call_params(CallParameters::new(
            amount,
            asset,
            self.gas_limit,
        )).unwrap()
        .call()
        .await
        .unwrap();
    }

    /*
      call get_deposits() function.
     */
    pub async fn call_get_deposits(self, depositor: Identity) -> (u64, u64) {
        let result = self.instance.clone()
        .with_account(self.wallet)
        .methods()
        .get_deposits(depositor)
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
    pub async fn call_withdraw(self, to: Identity) {
        self.instance.clone()
        .with_account(self.wallet)                   
        .methods()
        .withdraw(to)
        .with_variable_output_policy(VariableOutputPolicy::Exactly(2))
        .with_tx_policies(
            TxPolicies::default()
            .with_script_gas_limit(self.gas_limit)
        )
        .call()
        .await
        .unwrap();
    }

    pub async fn get_contract_balance(&self, asset_id: AssetId) -> u64 {
        let assets = self.instance.clone().get_balances().await.unwrap();
        if let Some(amount) = assets.get(&asset_id) {
            *amount
        } else {
            0
        }
    }
}
