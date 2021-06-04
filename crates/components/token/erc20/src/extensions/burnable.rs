use metis_lang::Env;

use crate::erc20::{Impl, Result};

/// Extension of {ERC20} that allows token holders to destroy both their own
/// tokens and those that they have an allowance for, in a way that can be
/// recognized off-chain (via event analysis).
pub trait ImplBurnable<E>: Impl<E>
where
    E: Env,
{
    /// @dev Destroys `amount` tokens from `account`, reducing the
    /// total supply.
    ///
    /// Emits a {Transfer} event with `to` set to the None address.
    ///
    /// Requirements:
    ///
    /// - `account` must have at least `amount` tokens.
    fn _burn(&mut self, account: &E::AccountId, amount: E::Balance) -> Result<()> {
        let account_balance = self.get().balance_of(account);
        let total_supply = self.get().total_supply();

        assert!(account_balance >= amount);
        self.get_mut()
            .set_balance(account, account_balance - amount);
        self.get_mut().set_total_supply(total_supply - amount);

        self.emit_event_transfer(Some(account.clone()), None, amount);

        Ok(())
    }

    /// Destroys `amount` tokens from the caller.
    fn burn(&mut self, amount: E::Balance) -> Result<()> {
        self._burn(&Self::caller(), amount)
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
    fn burn_from(&mut self, account: &E::AccountId, amount: E::Balance) -> Result<()> {
        let caller = &Self::caller();
        let current_allowance = self.get().allowance(account, caller);
        assert!(current_allowance >= amount);
        self._approve(account, caller, amount);
        self._burn(account, amount)
    }
}

// No impl this for default
// impl<E: Env, T: Storage<E> + EventEmit<E>> ImplBurnable<E> for T {}
