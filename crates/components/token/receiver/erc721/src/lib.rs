//! Contract components which provides a common implementation for erc721 receiver.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_prelude::vec::Vec;
use metis_erc721::TokenId;
use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    lazy::Lazy,
    traits::SpreadLayout,
};

#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data {
    v: Lazy<u8>,
}

impl Default for Data {
    fn default() -> Self {
        Self { v: Lazy::default() }
    }
}

/// The `EventEmit` impl the event emit api for erc721-receiver component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emit Erc721Received event
    fn emit_event_erc_721_received(
        &mut self,
        operator: E::AccountId,
        from: E::AccountId,
        token_id: TokenId,
        data: Vec<u8>,
    );
}

/// The `Impl` define erc721-receiver component impl funcs
pub trait Impl<E: Env>: EventEmit<E> + Storage<E, Data> {
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {}

    fn on_erc721_received(
        &mut self,
        operator: E::AccountId,
        from: E::AccountId,
        token_id: TokenId,
        data: Vec<u8>,
    ) -> [u8; 4] {
        Self::emit_event_erc_721_received(self, operator, from, token_id, data);

        metis_lang::selector_id!(on_erc721_received)
    }
}

impl<E: Env, T: EventEmit<E> + Storage<E, Data>> Impl<E> for T {}
