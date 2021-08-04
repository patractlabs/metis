//! Extension of {ERC20} that adds a cap to the supply of tokens.
use crate::{
    erc20::Result,
    Impl as ERC20,
};
use metis_lang::{
    Env,
    Storage,
};

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    lazy::Lazy,
    traits::SpreadLayout,
};

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// The cap on the token's total supply.
    cap: Lazy<E::Balance>,
}

impl<E: Env> Data<E> {
    /// Sets the value of the `cap`. This value is immutable, it can only be
    /// set once during construction.
    pub fn new(cap: E::Balance) -> Self {
        let mut res = Self::default();
        res.cap = Lazy::new(cap);
        res
    }
}

impl<E> Default for Data<E>
where
    E: Env,
{
    fn default() -> Self {
        Self {
            cap: Lazy::default(),
        }
    }
}

impl<E: Env> Data<E> {
    /// Returns the cap on the token's total supply.
    pub fn cap(&self) -> E::Balance {
        *self.cap
    }
}

/// Extension of {ERC20} that adds a cap to the supply of tokens.
pub trait Impl<E>: crate::hookable::Impl<E> + Storage<E, Data<E>>
where
    E: Env,
{
    /// Returns the cap on the token's total supply.
    fn cap(&self) -> E::Balance {
        Storage::<E, Data<E>>::get(self).cap()
    }

    /// See {ERC20-_mint}.
    fn _mint(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
        assert!(
            ERC20::total_supply(self) + amount <= self.cap(),
            "ERC20Capped: cap exceeded"
        );

        ERC20::_mint(self, account, amount)
    }
}
