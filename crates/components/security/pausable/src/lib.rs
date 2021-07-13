//! Contract module which allows children to implement an emergency stop
//! mechanism that can be triggered by an authorized account.
//!
//! This module is used through inheritance. It will make available the
//! api `ensure_not_paused` and `ensure_paused`, which can be applied to
//! the functions of your contract. Note that they will not be pausable by
//! simply including this module, only once the modifiers are put in place.

#![cfg_attr(not(feature = "std"), no_std)]

mod module;

use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

/// The `EventEmit` impl the event emit api for ownable component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emit Paused event
    fn emit_event_paused(&mut self, account: E::AccountId);

    /// Emit Unpaused event
    fn emit_event_unpaused(&mut self, account: E::AccountId);
}

/// The `Impl` define ownable component impl funcs
pub trait Impl<E: Env>: Storage<E, Data> + EventEmit<E> {
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {}

    /// Returns to normal state.
    ///
    /// Requirements:
    ///
    /// - The contract must be paused.
    fn _pause(&mut self) {
        self.ensure_not_paused();
        self.get_mut().pause();
        self.emit_event_paused(Self::caller());
    }

    /// Transfers ownership of the contract to a new account (`new_owner`).
    /// Can only be called by the current owner.
    fn _unpause(&mut self) {
        self.ensure_paused();
        self.get_mut().unpause();
        self.emit_event_unpaused(Self::caller());
    }

    /// Returns true if the contract is paused, and false otherwise
    fn paused(&self) -> bool {
        self.get().is_paused()
    }

    /// Panic if `owner` is not an owner
    fn ensure_paused(&self) {
        assert!(self.get().is_paused(), "Pausable: ensure paused");
    }

    /// Panic if caller is not an owner
    fn ensure_not_paused(&self) {
        assert!(!self.get().is_paused(), "Pausable: ensure not paused");
    }
}

impl<E: Env, T: Storage<E, Data> + EventEmit<E>> Impl<E> for T {}
