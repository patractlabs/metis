//! Contract components which provides a basic access control mechanism, where
//! there is an account (an owner) that can be granted exclusive access to
//! specific functions.
//!
//! By default, the owner account will be the one that deploys the contract. This
//! can later be changed with {transferOwnership}.
//!
//! This components is used through inheritance. It will make available the func like
//! `ensure_caller_is_owner`, which can be applied to your functions to restrict 
//! their use to the owner.

#![cfg_attr(not(feature = "std"), no_std)]

mod module;

pub use metis_lang::{Env, EnvAccess, Storage};
pub use module::Data;

/// The `EventEmit` impl the event emit api for ownable component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// emit_event_ownership_transferred emit OwnershipTransferred event
    fn emit_event_ownership_transferred(
        &mut self,
        previous_owner: Option<E::AccountId>,
        new_owner: Option<E::AccountId>,
    );
}

/// The `Impl` define ownable component impl funcs
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {
        self.get_mut().set_ownership(&Some(Self::caller()));
    }

    /// renounce_ownership Leaves the contract without owner. It will not be possible to call
    /// `ensure_xxx` functions anymore. Can only be called by the current owner.
    /// NOTE: Renouncing ownership will leave the contract without an owner,
    /// thereby removing any functionality that is only available to the owner.
    fn renounce_ownership(&mut self) {
        self.ensure_caller_is_owner();

        self.emit_event_ownership_transferred(self.get().get_ownership().clone(), None);

        self.get_mut().set_ownership(&None);
    }

    /// transfer_ownership Transfers ownership of the contract to a new account (`new_owner`).
    /// Can only be called by the current owner.
    fn transfer_ownership(&mut self, new_owner: &E::AccountId) {
        self.ensure_caller_is_owner();

        let new_owner_account = Some(new_owner.clone());

        self.emit_event_ownership_transferred(
            self.get().get_ownership().clone(),
            new_owner_account.clone(),
        );

        self.get_mut().set_ownership(&new_owner_account);
    }

    /// Return the owner AccountId
    fn owner(&self) -> &Option<E::AccountId> {
        self.get().get_ownership()
    }

    /// Panic if `owner` is not an owner
    fn ensure_owner(&self, owner: &E::AccountId) {
        assert!(&self.get().get_ownership().clone().unwrap() == owner);
    }

    /// Panic if caller is not an owner
    fn ensure_caller_is_owner(&self) {
        self.ensure_owner(&Self::caller());
    }

    /// Panic the contract owner is not renounced,
    fn ensure_owner_renounce(&self) {
        assert!(self.get().get_ownership().is_none());
    }
}

impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
