#![cfg_attr(not(feature = "std"), no_std)]

mod module;

pub use metis_contract::{Env, EnvAccess};

pub use module::{Data, Storage};

pub trait EventEmit<E: Env>: EnvAccess<E> {}

pub trait Impl<E: Env>: Storage<E> + EventEmit<E> {
    // logics
    fn init(&mut self) {
        let caller = Self::caller();
        self.get_mut().set_ownership(&Some(caller));
    }

    fn renounce_ownership(&mut self) {
        self.ensure_caller_is_owner();

        // Self::env().emit_event(OwnershipTransferred {
        //    previous_owner: *self.get_ownership(),
        //    new_owner: None,
        //});

        self.get_mut().set_ownership(&None);
    }

    fn transfer_ownership(&mut self, new_owner: E::AccountId) {
        self.ensure_caller_is_owner();

        // Self::env().emit_event(OwnershipTransferred {
        //    previous_owner: *self.get_ownership(),
        //    new_owner: Some(new_owner),
        //});

        self.get_mut().set_ownership(&Some(new_owner));
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

impl<E: Env, T: Storage<E> + EventEmit<E>> Impl<E> for T {}
