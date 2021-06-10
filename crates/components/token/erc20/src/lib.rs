//! Implementation of the ERC20 component.

#![cfg_attr(not(feature = "std"), no_std)]

mod erc20;
mod erc20_basic;
mod extensions;
mod module;

pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

pub use erc20_basic::{
    Error,
    EventEmit,
    Impl,
    Result,
};

pub mod default {
    pub use super::erc20::Impl;
}

pub mod hookable {
    pub use super::extensions::hookable::Impl;
}

pub mod pausable {
    pub use super::extensions::pausable::Impl;
}

pub mod burnable {
    pub use super::extensions::burnable::Impl;
}

pub mod capped {
    pub use super::extensions::capped::{
        Data,
        Impl,
    };
}

// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
