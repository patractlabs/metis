//! Contract module that helps prevent reentrant calls to a function.

#![cfg_attr(not(feature = "std"), no_std)]

mod module;

use metis_lang::{
    Env,
    Storage,
};

pub use module::Data;

/// The `Impl` define ownable component impl funcs
pub trait Impl<E: Env>: Storage<E, Data> {
    /// is_entered is current is paused
    fn _check_nonreentrant(&self) {
        assert!(!self.get().is_entered(), "ReentrancyGuard: reentrant call");
    }

    /// set current status to entered
    fn _set_entered(&mut self) {
        self.get_mut().set_entered();
    }

    /// set current status to not entered
    fn _set_not_entered(&mut self) {
        self.get_mut().set_not_entered();
    }
}

impl<E: Env, T: Storage<E, Data>> Impl<E> for T {}
