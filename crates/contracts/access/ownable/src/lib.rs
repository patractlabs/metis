#![cfg_attr(not(feature = "std"), no_std)]

pub use metis_contract::{Env, EnvAccess};

mod module;

pub use module::{Data, Storage};

pub trait EventEmit<E: Env>: EnvAccess<E> {
    fn emit_event_ownership_transferred(
        &mut self,
        previous_owner: Option<E::AccountId>,
        new_owner: Option<E::AccountId>,
    );
}

pub trait Impl<E: Env>: Storage<E> + EventEmit<E> {
    // logics
    fn init(&mut self) {
        self.get_mut().set_ownership(&Some(Self::caller()));
    }

    fn renounce_ownership(&mut self) {
        self.ensure_caller_is_owner();

        self.emit_event_ownership_transferred(
            self.get().get_ownership().clone(),
            None);

        self.get_mut().set_ownership(&None);
    }

    fn transfer_ownership(&mut self, new_owner: &E::AccountId) {
        self.ensure_caller_is_owner();

        let new_owner_account = Some(new_owner.clone());

        self.emit_event_ownership_transferred(
            self.get().get_ownership().clone(),
            new_owner_account.clone());

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

impl<E: Env, T: Storage<E> + EventEmit<E>> Impl<E> for T {}
