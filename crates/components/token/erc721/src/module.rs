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

use crate::types::TokenId;

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// Symbols of ERC20 Token, by (name, symbol)
    pub symbols: Lazy<(String, String)>,

    /// Mapping from token ID to owner address
    pub owners: StorageHashMap<TokenId, E::AccountId>,

    /// Mapping owner address to token count
    pub balances: StorageHashMap<E::AccountId, E::Balance>,

    /// Mapping from token ID to approved address
    pub token_approvals: StorageHashMap<TokenId, E::AccountId>,

    /// Mapping from owner to operator approvals
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
            symbols: Lazy::default(),
            owners: StorageHashMap::new(),
            balances: StorageHashMap::new(),
            token_approvals: StorageHashMap::new(),
            operator_approvals: StorageHashMap::new(),
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
        self.balances.get(owner).copied().unwrap_or(E::Balance::from(0_u8))
    }

    pub fn set_approval_for_all(
        &mut self,
        owner: E::AccountId,
        operator: E::AccountId,
        approved: bool,
    ) {
        self.operator_approvals.insert((owner, operator), approved);
    }

    pub fn is_approved_for_all(
        &self,
        owner: E::AccountId,
        operator: E::AccountId,
    ) -> bool {
        match self.operator_approvals.get(&(owner, operator)) {
            Some(approved) => approved.clone(),
            None => false,
        }
    }

    fn _mod_balance(&mut self, owner: &E::AccountId, is_add: bool) {
        match self.balances.entry(owner.clone()) {
            Entry::Vacant(vacant) => {
                assert!(is_add, "ERC721: balance can not dec to -1");
                vacant.insert(E::Balance::from(1_u8));
            }
            Entry::Occupied(mut occupied) => {
                let old_value = *occupied.get();
                assert!(
                    old_value != E::Balance::from(0_u8) || is_add,
                    "ERC721: balance can not dec to -1 from 0"
                );

                let new_value = match is_add {
                    true => old_value + E::Balance::from(1_u8),
                    false => old_value - E::Balance::from(1_u8),
                };

                occupied.insert(new_value);
            }
        };
    }

    pub fn balance_inc(&mut self, owner: &E::AccountId){
        self._mod_balance(owner, true);
    }

    pub fn balance_dec(&mut self, owner: &E::AccountId){
        self._mod_balance(owner, false);
    }
}