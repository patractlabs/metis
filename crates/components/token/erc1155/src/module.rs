use ink_prelude::string::String;
pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::{
        hashmap::Entry,
        HashMap as StorageHashMap,
    },
    lazy::Lazy,
    traits::SpreadLayout,
};

use crate::TokenId;

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// Symbols of ERC20 Token, by (name, symbol)
    pub url: Lazy<String>,

    /// Mapping from token ID to account balances
    pub balances: StorageHashMap<(TokenId, E::AccountId), E::Balance>,

    /// Mapping from account to operator approvals
    pub operator_approvals: StorageHashMap<(E::AccountId, E::AccountId), bool>,
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
            url: Lazy::default(),
            balances: StorageHashMap::new(),
            operator_approvals: StorageHashMap::new(),
        }
    }
}

impl<E: Env> Data<E> {
    /// Get the name and symbol of Token
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /// Set the name and symbol of Token
    pub fn set_url(&mut self, url: String) {
        Lazy::set(&mut self.url, url);
    }

    /// Return the balance of {account} in {id}
    pub fn balance_of(&self, id: &TokenId, account: &E::AccountId) -> E::Balance {
        self.balances
            .get(&(id.clone(), account.clone()))
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    /// Set the Balance of {account} in {id}
    pub fn set_balance(
        &mut self,
        account: &E::AccountId,
        id: &TokenId,
        balance: E::Balance,
    ) {
        let key = (id.clone(), account.clone());

        if balance == E::Balance::from(0_u8) {
            self.balances.take(&key);
        } else {
            self.balances.insert(key, balance);
        }
    }

    /// Add the Balance of {account} in {id} with {amount}
    pub fn add_balance(
        &mut self,
        account: &E::AccountId,
        id: &TokenId,
        amount: E::Balance,
    ) {
        match self.balances.entry((id.clone(), account.clone())) {
            Entry::Occupied(mut entry) => {
                let v = entry.get_mut();
                *v += amount;
            }
            Entry::Vacant(entry) => {
                entry.insert(amount);
            },
        };
    }

    pub fn set_approval_for_all(
        &mut self,
        owner: E::AccountId,
        operator: E::AccountId,
        approved: bool,
    ) {
        let key = (owner, operator);
        if !approved {
            match self.operator_approvals.entry(key.clone()) {
                Entry::Occupied(entry) => {
                    entry.remove();
                }
                Entry::Vacant(_) => {}
            }
        } else {
            self.operator_approvals.insert(key, approved);
        }
    }

    pub fn is_approved_for_all(
        &self,
        owner: &E::AccountId,
        operator: &E::AccountId,
    ) -> bool {
        match self
            .operator_approvals
            .get(&(owner.clone(), operator.clone()))
        {
            Some(approved) => approved.clone(),
            None => false,
        }
    }
}
