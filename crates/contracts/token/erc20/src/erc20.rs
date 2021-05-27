pub use super::module::{Data, Storage};
pub use metis_contract::{Env, EnvAccess};

/// The ERC-20 error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
}

/// The ERC-20 result type.
pub type Result<T> = core::result::Result<T, Error>;

pub trait EventEmit<E: Env>: EnvAccess<E> {
    fn emit_event_transfer(
        &mut self,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        value: E::Balance,
    );

    fn emit_event_approval(
        &mut self,
        owner: E::AccountId,
        spender: E::AccountId,
        value: E::Balance,
    );
}

pub trait Impl<E: Env>: Storage<E> + EventEmit<E> {
    fn _approve(&mut self, owner: E::AccountId, spender: E::AccountId, amount: E::Balance) {
        self.get_mut()
            .set_allowance((owner.clone(), spender.clone()), amount);
        self.emit_event_approval(owner, spender, amount);
    }

    // logics
    fn init(&mut self, initial_supply: E::Balance) {
        let caller = Self::caller();
        self.get_mut().set_total_supply(initial_supply);
        self.get_mut().set_balance(caller.clone(), initial_supply);

        self.emit_event_transfer(None, Some(caller), initial_supply);
    }

    fn transfer(&mut self, to: E::AccountId, value: E::Balance) -> Result<()> {
        self.transfer_from_to(Self::caller(), to, value)
    }

    fn approve(&mut self, spender: E::AccountId, amount: E::Balance) -> Result<()> {
        self._approve(Self::caller(), spender, amount);
        Ok(())
    }

    fn transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        value: E::Balance,
    ) -> Result<()> {
        let caller = Self::caller();
        let allowance = self.get().get_allowance(from.clone(), caller.clone());
        if allowance < value {
            return Err(Error::InsufficientAllowance);
        }
        self.transfer_from_to(from.clone(), to, value)?;
        self.get_mut()
            .set_allowance((from, caller), allowance - value);
        Ok(())
    }

    fn transfer_from_to(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        value: E::Balance,
    ) -> Result<()> {
        let from_balance = self.get().get_balance(from.clone());
        if from_balance < value {
            return Err(Error::InsufficientBalance);
        }
        self.get_mut()
            .set_balance(from.clone(), from_balance - value);
        let to_balance = self.get().get_balance(to.clone());
        self.get_mut().set_balance(to.clone(), to_balance + value);

        self.emit_event_transfer(Some(from), Some(to), value);

        Ok(())
    }
}
