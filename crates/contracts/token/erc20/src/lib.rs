#![cfg_attr(not(feature = "std"), no_std)]

mod module;
mod erc20;
mod erc20_burnable;

pub use metis_contract::{Env, EnvAccess};
pub use module::{Data, Storage};

pub use erc20::{
    Error,
    Result,
    EventEmit,
    Impl,
};

pub use erc20_burnable::{
    ImplBurnable
};

impl<E: Env, T: Storage<E> + EventEmit<E>> Impl<E> for T {}
