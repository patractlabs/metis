//! Contract module which acts as a timelocked controller. When set as the
//! owner of an `Ownable` smart contract, it enforces a timelock on all
//! `onlyOwner` maintenance operations. This gives time for users of the
//! controlled contract to exit before a potentially dangerous maintenance
//! operation is applied.
//!
//! By default, this contract is self administered, meaning administration tasks
//! have to go through the timelock process. The proposer (resp executor) role
//! is in charge of proposing (resp executing) operations. A common use case is
//! to position this {TimelockController} as the owner of a smart contract, with
//! a multisig or a DAO as the sole proposer.

#![cfg_attr(not(feature = "std"), no_std)]

pub use access_control::{
    Error,
    Result,
    RoleId,
};
use ink_env::hash::Blake2x256;
use ink_lang::ForwardCallMut;
use ink_prelude::{
    vec::Vec,
};
use metis_access_control as access_control;
use metis_lang::{
    Env,
    FromAccountId,
    Storage,
};
use metis_timelock_controller_receiver::Receiver;
use scale::Encode;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    lazy::Lazy,
    traits::SpreadLayout,
};

pub const TIMELOCK_ADMIN_ROLE: RoleId =
    RoleId::new(metis_lang::hash!(TIMELOCK_ADMIN_ROLE));
pub const PROPOSER_ROLE: RoleId = RoleId::new(metis_lang::hash!(PROPOSER_ROLE));
pub const EXECUTOR_ROLE: RoleId = RoleId::new(metis_lang::hash!(EXECUTOR_ROLE));
pub const _DONE_TIMESTAMP: u8 = 1;

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// min delay for controller
    pub min_delay: Lazy<E::Timestamp>,

    pub timestamps: StorageHashMap<[u8; 32], E::Timestamp>,
}

impl<E: Env> Data<E> {
    /// Sets the value of the `cap`. This value is immutable, it can only be
    /// set once during construction.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E> Default for Data<E>
where
    E: Env,
{
    fn default() -> Self {
        Self {
            min_delay: Lazy::new(E::Timestamp::from(1_u8)),
            timestamps: StorageHashMap::default(),
        }
    }
}

impl<E: Env> Data<E> {}

/// The `EventEmit` impl the event emit api for component.
pub trait EventEmit<E: Env> {
    /// Emitted when a call is scheduled as part of operation `id`.
    fn emit_event_call_scheduled(
        &mut self,
        id: [u8; 32],
        target: E::AccountId,
        value: E::Balance,
        data: Vec<u8>,
        predecessor: Option<[u8; 32]>,
        delay: E::Timestamp,
    );

    /// Emitted when a call is performed as part of operation `id`.
    fn emit_event_call_executed(
        &mut self,
        id: [u8; 32],
        target: E::AccountId,
        value: E::Balance,
        data: Vec<u8>,
    );

    /// Emitted when operation `id` is cancelled.
    fn emit_event_cancelled(&mut self, id: [u8; 32]);

    /// Emitted when the minimum delay for future operations is modified.
    fn emit_event_min_delay_change(
        &mut self,
        old_duration: E::Timestamp,
        new_duration: E::Timestamp,
    );
}

pub trait Impl<E>: access_control::Impl<E> + EventEmit<E> + Storage<E, Data<E>>
where
    E: Env,
{
    /// initial the state of contract
    fn init(
        &mut self,
        min_delay: E::Timestamp,
        proposers: Vec<E::AccountId>,
        executors: Vec<E::AccountId>,
    ) {
        access_control::Impl::_set_role_admin(
            self,
            TIMELOCK_ADMIN_ROLE,
            TIMELOCK_ADMIN_ROLE,
        );
        access_control::Impl::_set_role_admin(self, PROPOSER_ROLE, TIMELOCK_ADMIN_ROLE);
        access_control::Impl::_set_role_admin(self, EXECUTOR_ROLE, TIMELOCK_ADMIN_ROLE);

        // deployer + self administration
        access_control::Impl::_setup_role(self, TIMELOCK_ADMIN_ROLE, Self::caller());
        // access_control::Impl::_setup_role(self, TIMELOCK_ADMIN_ROLE, address(this));

        // register proposers
        for proposer in proposers.iter() {
            access_control::Impl::_setup_role(self, PROPOSER_ROLE, proposer.clone());
        }

        // register executors
        for executor in executors.iter() {
            access_control::Impl::_setup_role(self, EXECUTOR_ROLE, executor.clone());
        }

        Lazy::set(
            &mut Storage::<E, Data<E>>::get_mut(self).min_delay,
            min_delay,
        );
        self.emit_event_min_delay_change(E::Timestamp::from(0_u8), min_delay);
    }

    /// To make a function callable only by a certain role. In
    /// addition to checking the sender's role, `address(0)` 's role is also
    /// considered. Granting a role to `address(0)` is equivalent to enabling
    /// this role for everyone.
    fn ensure_only_role_or_open_role(&self, role: RoleId) {
        if !access_control::Impl::has_role(self, role, E::AccountId::default()) {
            access_control::Impl::ensure_caller_role(self, role);
        }
    }

    /// Returns whether an id correspond to a registered operation. This
    /// includes both Pending, Ready and Done operations.
    fn is_operation(&self, id: &[u8; 32]) -> bool {
        self.get_timestamp(id) > E::Timestamp::from(0_u8)
    }

    /// Returns whether an operation is pending or not.
    fn is_operation_pending(&self, id: &[u8; 32]) -> bool {
        self.get_timestamp(id) > E::Timestamp::from(_DONE_TIMESTAMP)
    }

    /// Returns whether an operation is ready or not.
    fn is_operation_ready(&self, id: &[u8; 32]) -> bool {
        let timestamp = self.get_timestamp(id);
        timestamp > E::Timestamp::from(_DONE_TIMESTAMP)
            && timestamp <= Self::block_timestamp()
    }

    /// Returns whether an operation is done or not.
    fn is_operation_done(&self, id: &[u8; 32]) -> bool {
        self.get_timestamp(id) == E::Timestamp::from(_DONE_TIMESTAMP)
    }

    /// Returns the timestamp at with an operation becomes ready (0 for
    /// unset operations, 1 for done operations).
    fn get_timestamp(&self, id: &[u8; 32]) -> E::Timestamp {
        *Storage::<E, Data<E>>::get(self)
            .timestamps
            .get(id)
            .unwrap_or(&E::Timestamp::from(0_u8))
    }

    /// Returns the minimum delay for an operation to become valid.
    ///
    /// This value can be changed by executing an operation that calls `updateDelay`.
    fn get_min_delay(&self) -> E::Timestamp {
        *Storage::<E, Data<E>>::get(self).min_delay
    }

    /// Returns the identifier of an operation containing a single
    /// transaction.
    fn hash_operation(
        &self,
        target: &E::AccountId,
        value: &E::Balance,
        data: &Vec<u8>,
        predecessor: &Option<[u8; 32]>,
        salt: &[u8; 32],
    ) -> [u8; 32] {
        // for target + value + data + predecessor + salt
        let mut hash_data: Vec<u8> = Vec::with_capacity(128 + data.len());

        hash_data.append(&mut target.encode());
        hash_data.append(&mut value.encode());
        hash_data.append(&mut data.clone());
        hash_data.append(&mut predecessor.encode());
        for s in salt.into_iter() {
            hash_data.push(s.clone());
        }

        Self::hash_bytes::<Blake2x256>(&hash_data)
    }

    /// Schedule an operation containing a single transaction.
    ///
    /// Emits a `CallScheduled` event.
    ///
    /// Requirements:
    ///
    /// - the caller must have the 'proposer' role.
    fn schedule(
        &mut self,
        target: E::AccountId,
        value: E::Balance,
        data: Vec<u8>,
        predecessor: Option<[u8; 32]>,
        salt: [u8; 32],
        delay: E::Timestamp,
    ) {
        access_control::Impl::ensure_caller_role(self, PROPOSER_ROLE);

        let id = self.hash_operation(&target, &value, &data, &predecessor, &salt);

        self._schedule(id, delay);

        self.emit_event_call_scheduled(id, target, value, data, predecessor, delay);
    }

    /// Schedule an operation that is to becomes valid after a given delay.
    fn _schedule(&mut self, id: [u8; 32], delay: E::Timestamp) {
        assert!(
            !self.is_operation(&id),
            "TimelockController: operation already scheduled"
        );
        assert!(
            delay >= self.get_min_delay(),
            "TimelockController: insufficient delay"
        );

        Storage::<E, Data<E>>::get_mut(self)
            .timestamps
            .insert(id, Self::block_timestamp() + delay);
    }

    /// Cancel an operation.
    ///
    /// Requirements:
    ///
    /// - the caller must have the 'proposer' role.
    fn cancel(&mut self, id: [u8; 32]) {
        access_control::Impl::ensure_caller_role(self, PROPOSER_ROLE);

        assert!(
            self.is_operation_pending(&id),
            "TimelockController: operation cannot be cancelled"
        );
        Storage::<E, Data<E>>::get_mut(self).timestamps.take(&id);

        self.emit_event_cancelled(id);
    }

    /// Execute an (ready) operation containing a single transaction.
    ///
    /// Emits a `CallExecuted` event.
    ///
    /// Requirements:
    ///
    /// - the caller must have the 'executor' role.
    fn execute(
        &mut self,
        target: E::AccountId,
        value: E::Balance,
        data: Vec<u8>,
        predecessor: Option<[u8; 32]>,
        salt: [u8; 32],
    ) {
        self.ensure_only_role_or_open_role(EXECUTOR_ROLE);

        let id = self.hash_operation(&target, &value, &data, &predecessor, &salt);

        self._before_call(predecessor);
        self._call(id, target, value, data);
        self._after_call(id);
    }

    /// Checks before execution of an operation's calls.
    fn _before_call(&self, predecessor: Option<[u8; 32]>) {
        match predecessor {
            Some(predecessor) => {
                assert!(
                    self.is_operation_done(&predecessor),
                    "TimelockController: missing dependency"
                );

                ()
            }
            None => (),
        }
    }

    /// Checks after execution of an operation's calls.
    fn _after_call(&mut self, id: [u8; 32]) {
        assert!(
            self.is_operation_ready(&id),
            "TimelockController: operation is not ready"
        );

        Storage::<E, Data<E>>::get_mut(self)
            .timestamps
            .insert(id, E::Timestamp::from(_DONE_TIMESTAMP));
    }

    /// Execute an operation's call.
    ///
    /// Emits a `CallExecuted` event.
    fn _call(
        &mut self,
        id: [u8; 32],
        target: E::AccountId,
        value: E::Balance,
        data: Vec<u8>,
    ) {
        let mut receiver =
            <Receiver as FromAccountId<E>>::from_account_id(target.clone());
        let success = receiver
            .call_mut()
            .on_call(Self::caller().into(), data.clone())
            .transferred_value(value.into())
            .fire();

        let success = match success {
            Ok(success) => success,
            Err(_) => false,
        };

        assert!(
            success,
            "TimelockController: underlying transaction reverted"
        );

        self.emit_event_call_executed(id, target, value, data);
    }

    /// Changes the minimum timelock duration for future operations.
    ///
    /// Emits a `MinDelayChange` event.
    ///
    /// Requirements:
    ///
    /// - the caller must be the timelock itself. This can only be achieved by scheduling and later executing
    /// an operation where the timelock is the target and the data is the ABI-encoded call to this fn.
    fn _set_update_delay(&mut self, new_delay: E::Timestamp) {
        let current_min_delay = self.get_min_delay();

        self.emit_event_min_delay_change(current_min_delay, new_delay);

        *Storage::<E, Data<E>>::get_mut(self).min_delay = new_delay;
    }
}
