#![cfg_attr(not(feature = "std"), no_std)]

mod erc20;
mod extensions;
mod module;

pub use metis_contract::{Env, EnvAccess};
pub use module::{Data, Storage};

pub use erc20::{Error, EventEmit, Impl, Result};

pub use extensions::burnable::ImplBurnable;

impl<E: Env, T: Storage<E> + EventEmit<E>> Impl<E> for T {}
