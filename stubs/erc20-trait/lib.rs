#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::{Erc20, StandardToken};
use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_lang as ink;
    use ink_prelude::string::String;

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, ()>;

    /// Trait implemented by all ERC-20 respecting smart contracts.
    #[ink::trait_definition]
    pub trait Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        fn new(initial_supply: Balance, name: String, symbol: String, decimals: u128) -> Self;

        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance;

        /// Returns the token name.
        #[ink(message)]
        fn token_name(&self) -> String;

        /// Returns the token symbol.
        #[ink(message)]
        fn token_symbol(&self) -> String;

        /// Returns the token decimals.
        #[ink(message)]
        fn token_decimals(&self) -> u128;

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
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>;

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>;
    }

    /// Basic version of StandardToken, with no allowances.
    #[ink(storage)]
    pub struct StandardToken {}

    impl Erc20 for StandardToken {
        #[ink(constructor)]
        fn new(_initial_supply: Balance, _name: String, _symbol: String, _decimals: u128) -> Self {
            unimplemented!()
        }

        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        /// Returns the token name.
        #[ink(message)]
        fn token_name(&self) -> String {
            unimplemented!()
        }

        /// Returns the token symbol.
        #[ink(message)]
        fn token_symbol(&self) -> String {
            unimplemented!()
        }

        /// Returns the token decimals.
        #[ink(message)]
        fn token_decimals(&self) -> u128 {
            unimplemented!()
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account balance of `from`.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<()> {
            unimplemented!()
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }
    }
}
