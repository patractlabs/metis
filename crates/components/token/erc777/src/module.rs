use ink_prelude::{
    string::String,
    vec::Vec,
};
pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    lazy::Lazy,
    traits::SpreadLayout,
};

/// The Data of ERC777 component
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
    /// Metadatas Symbols of ERC777 Token, by (name, symbol)
    pub metadatas: Lazy<(u8, String, String)>,

    /// This isn't ever read from - it's only used to respond to the defaultOperators query.
    pub default_operators_array: Lazy<Vec<E::AccountId>>,

    /// Immutable, but accounts may revoke them (tracked in __revokedDefaultOperators).
    pub default_operators: StorageHashMap<E::AccountId, ()>,

    /// For each account, a mapping of its operators and revoked default operators.
    pub operators: StorageHashMap<(E::AccountId, E::AccountId), ()>,
    pub revoked_default_operators: StorageHashMap<(E::AccountId, E::AccountId), ()>,
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
            default_operators_array: Lazy::default(),
            default_operators: StorageHashMap::new(),
            operators: StorageHashMap::new(),
            revoked_default_operators: StorageHashMap::new(),
        }
    }
}

impl<E: Env> Data<E> {
    /// init the data of erc777
    pub fn init(
        &mut self,
        name: String,
        symbol: String,
        default_operators: &Vec<E::AccountId>,
    ) {
        self.set_symbols(name, symbol, 18);
        for acc in default_operators {
            self.default_operators.insert(acc.clone(), ());
        }
        Lazy::set(&mut self.default_operators_array, default_operators.clone());
    }

    /// Get name of the ERC777 Token
    pub fn name(&self) -> &String {
        &self.metadatas.1
    }

    /// Get symbol of the ERC777 Token
    pub fn symbol(&self) -> &String {
        &self.metadatas.2
    }

    /// Get decimals of the ERC777 Token
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

    /// @dev Returns true if an account is an operator of `token_holder`.
    /// Operators can send and burn tokens on behalf of their owners. All
    /// accounts are their own operator.
    ///
    /// See {operatorSend} and {operatorBurn}.
    pub fn is_operator_for(
        &self,
        operator: &E::AccountId,
        token_holder: &E::AccountId,
    ) -> bool {
        let key = (token_holder.clone(), operator.clone());
        operator == token_holder
            || (self.default_operators.contains_key(&operator)
                && !self.revoked_default_operators.contains_key(&key))
            || self.operators.contains_key(&key)
    }

    /// Return {account} is a default operator
    pub fn is_default_operator(&self, account: &E::AccountId) -> bool {
        self.default_operators.contains_key(account)
    }
}
