//! Implementation of the ERC20 component.

#![cfg_attr(not(feature = "std"), no_std)]

mod basic;
mod extensions;
mod module;

pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

pub use metis_erc721_types::TokenId;

pub use basic::{
    Error,
    EventEmit,
    Impl,
    Result,
};

pub mod default {
    pub use super::basic::Impl;
}

pub mod burnable {
    pub use super::extensions::burnable::{
        Impl,
    };
}

pub mod enumerable {
    pub use super::extensions::enumerable::{
        Data,
        Impl,
    };
}

pub mod pausable {
    pub use super::extensions::pausable::{
        Impl,
    };
}
// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
