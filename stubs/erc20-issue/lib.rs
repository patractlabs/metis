#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::StandardToken;
use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use ink_prelude::string::String;

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Basic version of StandardToken, with no allowances.
    #[ink(storage)]
    pub struct StandardToken {}

    impl StandardToken {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor, selector = "0x2ee504f0")]
        pub fn new(
            _initial_supply: Balance,
            _name: String,
            _symbol: String,
            _decimals: u128,
        ) -> Self {
            unimplemented!()
        }

        /// Returns the total token supply.
        #[ink(message, selector = "0x3ef71755")]
        pub fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        /// Returns the token name.
        #[ink(message, selector = "0xba2d1c0b")]
        pub fn token_name(&self) -> String {
            unimplemented!()
        }

        /// Returns the token symbol.
        #[ink(message, selector = "0x58af15a8")]
        pub fn token_symbol(&self) -> String {
            unimplemented!()
        }

        /// Returns the token decimals.
        #[ink(message, selector = "0x55d2c725")]
        pub fn token_decimals(&self) -> u128 {
            unimplemented!()
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message, selector = "0x03de5509")]
        pub fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message, selector = "0x46607e68")]
        pub fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message, selector = "0x3a272874")]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message, selector = "0x5b396ade")]
        pub fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<()> {
            unimplemented!()
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message, selector = "0xc6703339")]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Issue a new amount of tokens
        /// these tokens are deposited into the owner address
        #[ink(message, selector = "0xc392ba4d")]
        pub fn issue(&mut self, _user: AccountId, _amount: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Redeem tokens.
        /// These tokens are withdrawn from the owner address
        /// if the balance must be enough to cover the redeem
        /// or the call will fail.
        #[ink(message, selector = "0xec3e9290")]
        pub fn redeem(&mut self, _user: AccountId, _amount: Balance) -> Result<()> {
            unimplemented!()
        }
    }
}
