#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::{
    Error,
    IErc20,
    Result,
};
pub mod events {
    // pub use crate::erc20::{Transfer, Approval};
}

use ink_lang as ink;
#[ink::contract]
mod erc20 {
    use ink_lang as ink;
    use ink_prelude::string::String;
    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        #[ink(topic)]
        pub value: Balance,
    }
    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        #[ink(topic)]
        pub value: Balance,
    }

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// Trait implemented by all ERC-20 respecting smart contracts.
    #[ink::trait_definition]
    pub trait IErc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        fn new(
            initial_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimals: Option<u8>,
        ) -> Self;

        /// Returns the token name.
        #[ink(message)]
        fn token_name(&self) -> Option<String>;

        /// Returns the token symbol.
        #[ink(message)]
        fn token_symbol(&self) -> Option<String>;

        /// Returns the token decimals.
        #[ink(message)]
        fn token_decimals(&self) -> Option<u8>;
        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance;
        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance;

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>;

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()>;

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>;
    }

    // TODO tmp hack struct for passing compile
    #[ink(storage)]
    pub struct Phantom;
    impl Phantom {
        #[ink(constructor)]
        pub fn new() -> Self {
            Phantom {}
        }
        #[ink(message)]
        pub fn message(&self) {}
    }
}
