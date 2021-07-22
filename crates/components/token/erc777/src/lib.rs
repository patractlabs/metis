//! Implementation of the ERC20 component.

#![cfg_attr(not(feature = "std"), no_std)]

mod basic;
mod module;

pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

pub use basic::{
    Error,
    EventEmit,
    Impl,
    Result,
};

pub mod default {
    pub use super::Impl;
}

// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
