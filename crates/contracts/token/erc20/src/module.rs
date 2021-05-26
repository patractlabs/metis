pub use metis_contract::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{collections::HashMap as StorageHashMap, lazy::Lazy, traits::SpreadLayout};

#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// Total token supply.
    total_supply: Lazy<E::Balance>,
    /// Mapping from owner to number of owned token.
    balances: StorageHashMap<E::AccountId, E::Balance>,
    /// Mapping of the token amount which an account is allowed to withdraw
    /// from another account.
    allowances: StorageHashMap<(E::AccountId, E::AccountId), E::Balance>,
}

pub trait Storage<E: Env> {
    fn get(&self) -> &Data<E>;
    fn get_mut(&mut self) -> &mut Data<E>;
}

impl<E: Env> Data<E> {
    pub fn new() -> Self {
        Self {
            total_supply: Lazy::default(),
            balances: StorageHashMap::new(),
            allowances: StorageHashMap::new(),
        }
    }
}

impl<E: Env> Data<E> {
    pub fn get_balance(&self, owner: E::AccountId) -> E::Balance {
        self.balances
            .get(&owner)
            .copied()
            .unwrap_or(E::Balance::from(0 as u8))
    }

    pub fn get_allowance(&self, owner: E::AccountId, spender: E::AccountId) -> E::Balance {
        self.allowances
            .get(&(owner, spender))
            .copied()
            .unwrap_or(E::Balance::from(0 as u8))
    }

    pub fn get_total_supply(&self) -> E::Balance {
        *self.total_supply
    }

    pub fn set_total_supply(&mut self, total_supply: E::Balance) {
        Lazy::set(&mut self.total_supply, total_supply);
    }

    pub fn balance_insert(&mut self, owner: E::AccountId, value: E::Balance) {
        self.balances.insert(owner, value);
    }

    pub fn allowance_insert(
        &mut self,
        owner_spender: (E::AccountId, E::AccountId),
        value: E::Balance,
    ) {
        self.allowances.insert(owner_spender, value);
    }
}
