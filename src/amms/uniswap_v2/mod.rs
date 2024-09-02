use super::{error::AMMError, AutomatedMarketMaker};

use alloy::{
    network::Network,
    primitives::{Address, B256, U256},
    providers::Provider,
    transports::Transport,
};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniswapV2Pool {}

impl AutomatedMarketMaker for UniswapV2Pool {
    fn address(&self) -> Address {
        todo!()
    }

    async fn sync<T, N, P>(&mut self, middleware: Arc<P>) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        todo!()
    }

    fn sync_signatures(&self) -> Vec<B256> {
        todo!()
    }

    fn simulate_swap(
        &self,
        base_token: Address,
        quote_token: Address,
        amount_in: U256,
    ) -> Result<U256, AMMError> {
        todo!()
    }

    fn simulate_swap_mut(
        &mut self,
        base_token: Address,
        quote_token: Address,
        amount_in: U256,
    ) -> Result<U256, AMMError> {
        todo!()
    }

    fn tokens(&self) -> Vec<Address> {
        todo!()
    }

    fn calculate_price(&self, base_token: Address, quote_token: Address) -> Result<f64, AMMError> {
        todo!()
    }
}
