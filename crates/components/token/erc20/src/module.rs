pub use metis_lang::Env;
use ink_prelude::string::String;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    lazy::Lazy,
    traits::SpreadLayout
};

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// Total token supply.
    pub total_supply: Lazy<E::Balance>,
    /// Mapping from owner to number of owned token.
    pub balances: StorageHashMap<E::AccountId, E::Balance>,
    /// Mapping of the token amount which an account is allowed to withdraw
    /// from another account.
    pub allowances: StorageHashMap<(E::AccountId, E::AccountId), E::Balance>,
    /// Symbols of ERC20 Token, by (name, symbol)
    pub symbols: Lazy<(String, String)>,
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
            symbols: Lazy::default(),
        }
    }
}

impl<E: Env> Data<E> {
    /// Get name of the ERC20 Token
    pub fn name(&self) -> &String {
        &self.symbols.0
    }

    /// Get symbol of the ERC20 Token
    pub fn symbol(&self) -> &String {
        &self.symbols.1
    }

    /// Set the name and symbol of Token
    pub fn set_symbols(&mut self, name: String, symbol: String) {
        Lazy::set(&mut self.symbols, (name, symbol));
    }

    /// Return the balance of {owner}
    pub fn balance_of(&self, owner: &E::AccountId) -> E::Balance {
        self.balances
            .get(owner)
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    /// Returns the allowance from {owner} to {spender}
    pub fn allowance(&self, owner: &E::AccountId, spender: &E::AccountId) -> E::Balance {
        self.allowances
            .get(&(owner.clone(), spender.clone()))
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    /// Return the total supply of token
    pub fn total_supply(&self) -> E::Balance {
        *self.total_supply
    }

    /// Set the total supply
    pub fn set_total_supply(&mut self, total_supply: E::Balance) {
        Lazy::set(&mut self.total_supply, total_supply);
    }

    /// Set the owner balance
    pub fn set_balance(&mut self, owner: &E::AccountId, value: E::Balance) {
        self.balances.insert(owner.clone(), value);
    }

    /// Set the allowance from owner to spender
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
