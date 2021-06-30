//! Escrow
//! Base escrow contract, holds funds designated for a payee until they
//! withdraw them.
//!
//! Intended usage: This contract (and derived escrow contracts) should be a
//! standalone contract, that only interacts with the contract that instantiated
//! it. That way, it is guaranteed that all Ether will be handled according to
//! the `Escrow` rules, and there is no need to check for payable functions or
//! transfers in the inheritance tree. The contract that uses the escrow as its
//! payment method should be its owner, and provide public methods redirecting
//! to the escrow's deposit and withdraw.

#![cfg_attr(not(feature = "std"), no_std)]

mod module;

use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};
use metis_ownable::Impl as Ownable;

pub use module::Data;

/// The `EventEmit` impl the event emit api for ownable component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emit Deposited event
    fn emit_event_deposited(
        &mut self,
        payee: E::AccountId,
        amount: E::Balance,
    );

    /// Emit Withdrawn event
    fn emit_event_withdrawn(
        &mut self,
        payee: E::AccountId,
        amount: E::Balance,
    );
}

/// The `Impl` define ownable component impl funcs
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> + Ownable<E> {
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {
    }

    /// Return the deposits of payee
    fn deposits_of(&self, payee: &E::AccountId) -> E::Balance {
        Storage::<E, Data<E>>::get(self).get(payee)
    }

    /// @dev Stores the sent amount as credit to be withdrawn.
    /// @param payee The destination address of the funds.
    fn deposit(&mut self, payee: E::AccountId) {
        self.ensure_caller_is_owner();

        let amount = Self::transferred_balance();

        Storage::<E, Data<E>>::get_mut(self).add(&payee, &amount);

        self.emit_event_deposited(payee, amount);
    }

    /// @dev Withdraw accumulated balance for a payee, forwarding all gas to the
    /// recipient.
    ///
    /// WARNING: Forwarding all gas opens the door to reentrancy vulnerabilities.
    /// Make sure you trust the recipient, or are either following the
    /// checks-effects-interactions pattern or using {ReentrancyGuard}.
    ///
    /// @param payee The address whose funds will be withdrawn and transferred to.
    fn withdraw(&mut self, payee: E::AccountId) {
        self.ensure_caller_is_owner();

        let payment = Storage::<E, Data<E>>::get(self).get(&payee);

        Storage::<E, Data<E>>::get_mut(self).clean(&payee);

        let res = Self::transfer(payee.clone(), payment);
        assert!(res.is_ok(), "Escrow: transfer to payee error");

        self.emit_event_withdrawn(payee, payment);
    }
}

impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E> + Ownable<E>> Impl<E> for T {}
