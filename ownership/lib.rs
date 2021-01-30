#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_lang as ink;

/// # Example: Implementation
/// ```
/// use ink_lang as ink;
///
/// #[ink::contract]
/// mod ownership {
/// 	use super::Ownable;
///
///     #[ink(storage)]
///     pub struct Ownership {
///         owner: AccountId,
///     }
///
///     impl Ownable for Ownership {
///         #[ink(constructor)]
///         fn new() -> Self {
///             Self {
///                 owner: Self::env().caller(),
///             }
///         }
///
///         #[ink(message)]
///         fn owner(&self) -> AccountId {
///             self.owner
///         }
///
///         #[ink(message)]
///         fn only_owner(&self) {
///             assert_eq!(self.env().caller(), self.owner);
///         }
///
///         #[ink(message)]
///         fn transfer_ownership(&mut self, new_owner: AccountId) {
///             self.only_owner();
///             assert_ne!(new_owner, Default::default());
///             self.owner = new_owner;
///         }
///
///         #[ink(message)]
///         fn renounce_ownership(&mut self) {
///             self.only_owner();
///             self.owner = Default::default();
///         }
///     }
/// }
/// ```

#[ink::trait_definition]
pub trait Ownable {
    /// Initializes the contract setting the deployer as the initial owner.
    #[ink(constructor)]
    fn new() -> Self;

    /// Returns the account id of the current owner.
    #[ink(message)]
    fn owner(&self) -> AccountId;

    /// Only owner has permission to access
    #[ink(message)]
    fn only_owner(&self);

    /// Transfer ownership to new owner.
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: AccountId);

    /// Leave the contract without an owner,
    /// thereby removing any functionality that is only available to the owner.
    #[ink(message)]
    fn renounce_ownership(&mut self);
}
