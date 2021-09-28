use ink_prelude::string::String;
pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    lazy::Lazy,
    traits::SpreadLayout,
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
    /// Metadatas Symbols of ERC20 Token, by (name, symbol)
    pub metadatas: Lazy<(u8, String, String)>,
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
            metadatas: Lazy::default(),
        }
    }
}

impl<E: Env> Data<E> {
    /// Get name of the ERC20 Token
    pub fn name(&self) -> &String {
        &self.metadatas.1
    }

    /// Get symbol of the ERC20 Token
    pub fn symbol(&self) -> &String {
        &self.metadatas.2
    }

    /// Get decimals of the ERC20 Token
    pub fn decimals(&self) -> &u8 {
        &self.metadatas.0
    }

    /// Set the name and symbol of Token
    pub fn set_symbols(&mut self, name: String, symbol: String, decimals: u8) {
        Lazy::set(&mut self.metadatas, (decimals, name, symbol));
    }

    /// Return the balance of {owner}
    pub fn balance_of(&self, owner: &E::AccountId) -> E::Balance {
        self.balances
            .get(owner)
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    /// Returns the allowance from {owner} to {spender}
    pub fn allowance(&self, owner: E::AccountId, spender: E::AccountId) -> E::Balance {
        self.allowances
            .get(&(owner, spender))
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
    pub fn set_balance(&mut self, owner: E::AccountId, value: E::Balance) {
        self.balances.insert(owner, value);
    }

    /// Set the allowance from owner to spender
    pub fn set_allowance(
        &mut self,
        owner: E::AccountId,
        spender: E::AccountId,
        value: E::Balance,
    ) {
        self.allowances.insert((owner, spender), value);
    }
}
