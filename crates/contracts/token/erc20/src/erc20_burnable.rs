pub use metis_contract::{Env, EnvAccess};
pub use super::module::{Data, Storage};

pub use super::erc20::{
    Error,
    Result,
    EventEmit,
    Impl,
};

pub trait ImplBurnable<E>: Impl<E> where E: Env{
    fn _burn(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
        let account_balance = self.get().get_balance(account.clone());
        let total_supply = self.get().get_total_supply();

        assert!(account_balance >= amount);
        self.get_mut().balance_insert(account.clone(), account_balance - amount);
        self.get_mut().set_total_supply(total_supply - amount);

        self.emit_event_transfer(Some(account), None, amount);

        Ok(())
    }

    fn burn(&mut self, amount: E::Balance) -> Result<()> {
        let caller = Self::caller();
        self._burn(caller, amount)
    }

    fn burn_from(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
        let caller = Self::caller();
        let current_allowance = self.get().get_allowance(account.clone(), caller.clone());
        assert!(current_allowance >= amount);
        self._approve(account.clone(), caller.clone(), amount);
        self._burn(account, amount)
    }
}

// No impl this for default
// impl<E: Env, T: Storage<E> + EventEmit<E>> ImplBurnable<E> for T {}