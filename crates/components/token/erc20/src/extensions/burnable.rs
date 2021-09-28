use metis_lang::Env;

use crate::erc20::{
    Error,
    Result,
};

/// Extension of {ERC20} that allows token holders to destroy both their own
/// tokens and those that they have an allowance for, in a way that can be
/// recognized off-chain (via event analysis).
pub trait Impl<E>: crate::hookable::Impl<E>
where
    E: Env,
{
    /// Destroys `amount` tokens from the caller.
    fn burn(&mut self, amount: E::Balance) -> Result<()> {
        self._burn(Self::caller(), amount)
    }

    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See {ERC20-_burn} and {ERC20-allowance}.
    ///
    /// Requirements:
    ///
    /// - the caller must have allowance for ``accounts``'s tokens of at least
    /// `amount`.
    fn burn_from(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
        let caller = Self::caller();
        let current_allowance = self.get().allowance(account.clone(), caller.clone());
        if current_allowance < amount {
            return Err(Error::InsufficientAllowance)
        }

        self._approve(account.clone(), caller, amount)?;

        self._burn(account, amount)
    }
}

// No impl this for default
// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> ImplBurnable<E> for T {}
