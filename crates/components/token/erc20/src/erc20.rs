pub use super::{
    module::Data,
    Error,
    EventEmit,
    Result,
};
use metis_lang::Env;

/// The `Impl` define erc20 component impl funcs
/// To Use this, should impl it:
///
/// impl metis_erc20::default::Impl<Contract> for Contract {}
pub trait Impl<E>: super::Impl<E>
where
    E: Env,
{
}

// TODO: default impl

// No impl this for default
// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> ImplBurnable<E> for T {}
