//! Implementation of the ERC1155 component.

#![cfg_attr(not(feature = "std"), no_std)]

mod basic;
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

// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
