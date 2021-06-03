pub use metis_lang::Env;

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

impl<E: Env> Data<E> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E> Default for Data<E>
where
    E: Env,
{
    fn default() -> Self {
        Self {
            total_supply: Lazy::default(),
            balances: StorageHashMap::new(),
            allowances: StorageHashMap::new(),
        }
    }
}

impl<E: Env> Data<E> {
    pub fn balance_of(&self, owner: &E::AccountId) -> E::Balance {
        self.balances
            .get(owner)
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    pub fn allowance(&self, owner: &E::AccountId, spender: &E::AccountId) -> E::Balance {
        self.allowances
            .get(&(owner.clone(), spender.clone()))
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    pub fn total_supply(&self) -> E::Balance {
        *self.total_supply
    }

    pub fn set_total_supply(&mut self, total_supply: E::Balance) {
        Lazy::set(&mut self.total_supply, total_supply);
    }

    pub fn set_balance(&mut self, owner: &E::AccountId, value: E::Balance) {
        self.balances.insert(owner.clone(), value);
    }

    pub fn set_allowance(
        &mut self,
        owner: &E::AccountId,
        spender: &E::AccountId,
        value: E::Balance,
    ) {
        self.allowances
            .insert((owner.clone(), spender.clone()), value);
    }
}
