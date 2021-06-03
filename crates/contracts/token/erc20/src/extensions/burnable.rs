pub use crate::module::Data;
pub use metis_lang::{Env, EnvAccess, Storage};

pub use crate::erc20::{Error, EventEmit, Impl, Result};

pub trait ImplBurnable<E>: Impl<E>
where
    E: Env,
{
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

    fn burn(&mut self, amount: E::Balance) -> Result<()> {
        self._burn(&Self::caller(), amount)
    }

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
