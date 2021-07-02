use ink_prelude::string::String;
pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    lazy::Lazy,
    traits::SpreadLayout,
};

use crate::TokenId;

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// Symbols of ERC20 Token, by (name, symbol)
    pub symbols: Lazy<(String, String)>,

    /// Mapping from token ID to owner address
    pub owners: StorageHashMap<TokenId, E::AccountId>,

    /// Mapping owner address to token count
    pub balances: StorageHashMap<E::AccountId, u64>,

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
    pub fn balance_of(&self, owner: &E::AccountId) -> u64 {
        self.balances
            .get(owner)
            .copied()
            .unwrap_or(0_u64)
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

    pub fn balance_inc(&mut self, owner: &E::AccountId) {
        let entry = self.balances.entry(owner.clone());

        entry
            .and_modify(|v| *v += 1_u64)
            .or_insert(1_u64);
    }

    pub fn balance_dec(&mut self, owner: &E::AccountId) {
        let count = self
            .balances
            .get_mut(owner)
            .expect("ERC721: balance not found");
        *count -= 1_u64;
    }
}
