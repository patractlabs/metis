//! Implementation of the ERC20 component.

#![cfg_attr(not(feature = "std"), no_std)]

mod erc20;
mod erc20_hookable;
mod extensions;
mod module;

pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

pub use erc20_hookable::{
    Error,
    EventEmit,
    Result,
    Impl,
};

pub mod default {
    pub use super::erc20::Impl;
}

pub mod hookable {
    pub use super::erc20_hookable::Impl;
}

pub mod burnable {
    pub use super::extensions::burnable::Impl;
}

// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
