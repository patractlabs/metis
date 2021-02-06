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
///         owner: Option<AccountId>,
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
///         fn owner(&self) -> Option<AccountId> {
///             self.owner.clone()
///         }
///
///         #[ink(message)]
///         fn transfer_ownership(&mut self, new_owner: Option<AccountId>) {
///             assert_eq!(self.owner(), Some(self.env().caller()));
///             if let Some(new_one) = new_owner {
///
///             }
///             self.owner = new_owner;
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
    fn owner(&self) -> Option<AccountId>;

    /// Transfer ownership to new owner.
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>);
}
