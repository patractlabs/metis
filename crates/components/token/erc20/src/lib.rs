//! Implementation of the ERC20 component.

#![cfg_attr(not(feature = "std"), no_std)]

mod erc20;
mod extensions;
mod module;

pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

pub use erc20::{
    Error,
    EventEmit,
    Impl,
    Result,
};

pub mod burnable {
    pub use super::extensions::burnable::Impl;
}

impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
