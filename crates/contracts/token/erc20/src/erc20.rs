pub use super::module::{Data};
pub use metis_contract::{Env, EnvAccess, Storage};

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

pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    fn _approve(&mut self, owner: &E::AccountId, spender: &E::AccountId, amount: E::Balance) {
        self.get_mut().set_allowance(owner, spender, amount);
        self.emit_event_approval(owner.clone(), spender.clone(), amount);
    }

    // logics
    fn init(&mut self, initial_supply: E::Balance) {
        let caller = &Self::caller();
        self.get_mut().set_total_supply(initial_supply);
        self.get_mut().set_balance(caller, initial_supply);

        self.emit_event_transfer(None, Some(caller.clone()), initial_supply);
    }

    fn transfer(&mut self, to: &E::AccountId, value: E::Balance) -> Result<()> {
        self._transfer_from_to(&Self::caller(), to, value)
    }

    fn approve(&mut self, spender: &E::AccountId, amount: E::Balance) -> Result<()> {
        self._approve(&Self::caller(), spender, amount);
        Ok(())
    }

    fn transfer_from(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let caller = &Self::caller();

        let current_allowance = self.get().allowance(from, caller);
        if current_allowance < amount {
            return Err(Error::InsufficientAllowance);
        }

        self._transfer_from_to(from, to, amount)?;

        self._approve(from, caller, current_allowance - amount);

        Ok(())
    }

    fn _transfer_from_to(
        &mut self,
        sender: &E::AccountId,
        recipient: &E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let sender_balance = self.get().balance_of(sender);
        if sender_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        self.get_mut().set_balance(sender, sender_balance - amount);
        let recipient_balance = self.get().balance_of(recipient);
        self.get_mut()
            .set_balance(recipient, recipient_balance + amount);

        self.emit_event_transfer(Some(sender.clone()), Some(recipient.clone()), amount);

        Ok(())
    }
}
