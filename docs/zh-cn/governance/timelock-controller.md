# Timelock Controller

Contract module which acts as a timelocked controller. When set as the
owner of an `Ownable` smart contract, it enforces a timelock on all
`enuser_caller_owner` maintenance operations. This gives time for users of the
controlled contract to exit before a potentially dangerous maintenance
operation is applied.

By default, this contract is self administered, meaning administration tasks
have to go through the timelock process. The proposer (resp executor) role
is in charge of proposing (resp executing) operations. A common use case is
to position this `TimelockController` as the owner of a smart contract, with
a multisig or a DAO as the sole proposer.

## Usage

To make a timelock controller contract, we should import timelock_controller at first.

Note the `timelock_controller` component is based on `access_control` component:

```rust
#[metis_lang::contract]
pub mod contract {
    use access_control::RoleId;
    use ink_prelude::vec::Vec;
    use metis_access_control as access_control;
    use metis_lang::{
        import,
        metis,
    };
    use metis_timelock_controller as timelock_controller;
    pub use metis_timelock_controller::{
        Error,
        Result,
    };

    #[ink(storage)]
    #[import(timelock_controller, access_control)]
    pub struct TimelockController {
        timelock_controller: timelock_controller::Data<TimelockController>,
        access_control: access_control::Data<TimelockController>,
    }

    // other logic for
}
```

Then implement the component:

```rust
    #[cfg(not(feature = "ink-as-dependency"))]
    impl timelock_controller::Impl<TimelockController> for TimelockController {}
```

Then add the event for timelock_controller, we add the events for the `access_control` component also:

```rust
    /// Emitted when a call is scheduled as part of operation `id`.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct CallScheduled {
        #[ink(topic)]
        pub id: [u8; 32],
        pub target: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
        pub predecessor: Option<[u8; 32]>,
        pub delay: Timestamp,
    }

    /// Emitted when a call is performed as part of operation `id`.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct CallExecuted {
        #[ink(topic)]
        pub id: [u8; 32],
        pub target: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
    }

    /// Emitted when operation `id` is cancelled.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct Cancelled {
        #[ink(topic)]
        pub id: [u8; 32],
    }

    /// Emitted when the minimum delay for future operations is modified.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct MinDelayChange {
        pub old_duration: Timestamp,
        pub new_duration: Timestamp,
    }

    /// Emitted when `new_admin_role` is set as ``role``'s
    /// admin role, replacing `previous_admin_role`
    #[ink(event)]
    #[metis(access_control)]
    pub struct RoleAdminChanged {
        #[ink(topic)]
        pub role: RoleId,
        #[ink(topic)]
        pub previous_admin_role: Option<RoleId>,
        #[ink(topic)]
        pub new_admin_role: RoleId,
    }

    /// Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call,
    /// an admin role bearer except when using {_setupRole}.
    #[ink(event)]
    #[metis(access_control)]
    pub struct RoleGranted {
        #[ink(topic)]
        pub role: RoleId,
        #[ink(topic)]
        pub account: AccountId,
        #[ink(topic)]
        pub sender: AccountId,
    }

    /// Emitted when `account` is revoked `role`.
    ///
    /// `sender` is the account that originated the contract call:
    ///   - if using `revoke_role`, it is the admin role bearer
    ///   - if using `renounce_role`, it is the role bearer (i.e. `account`)
    #[ink(event)]
    #[metis(access_control)]
    pub struct RoleRevoked {
        #[ink(topic)]
        pub role: RoleId,
        #[ink(topic)]
        pub account: AccountId,
        #[ink(topic)]
        pub sender: AccountId,
    }
```

impl the constructor for contract:

```rust
    impl TimelockController {
        #[ink(constructor)]
        pub fn new(
            min_delay: Timestamp,
            proposers: Vec<AccountId>,
            executors: Vec<AccountId>,
        ) -> Self {
            let mut instance = Self {
                timelock_controller: timelock_controller::Data::new(),
                access_control: access_control::Data::new(),
            };

            timelock_controller::Impl::init(
                &mut instance,
                min_delay,
                proposers,
                executors,
            );
            instance
        }
    }
```

Then implement the messages for contract.

> NOTE: the `execute` message should be `payable`

```rust
    impl TimelockController{
        /// Returns `true` if `account` has been granted `role`.
        #[ink(message)]
        pub fn has_role(&self, role: RoleId, account: AccountId) -> bool {
            access_control::Impl::has_role(self, role, account)
        }

        /// @dev Returns the admin role that controls `role`. See {grant_role} and
        /// {revoke_role}.
        ///
        /// To change a role's admin, use {_setRoleAdmin}.
        #[ink(message)]
        pub fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
            access_control::Impl::get_role_admin(self, role)
        }

        /// @dev Grants `role` to `account`.
        ///
        /// If `account` had not been already granted `role`, emits a {RoleGranted}
        /// event.
        ///
        /// Requirements:
        ///
        /// - the caller must have ``role``'s admin role.
        #[ink(message)]
        pub fn grant_role(&mut self, role: RoleId, account: AccountId) {
            access_control::Impl::grant_role(self, role, account)
        }

        /// @dev Revokes `role` from `account`.
        ///
        /// If `account` had been granted `role`, emits a {RoleRevoked} event.
        ///
        /// Requirements:
        ///
        /// - the caller must have ``role``'s admin role.
        #[ink(message)]
        pub fn revoke_role(&mut self, role: RoleId, account: AccountId) {
            access_control::Impl::revoke_role(self, role, account)
        }

        /// @dev Revokes `role` from the calling account.
        ///
        /// Roles are often managed via {grant_role} and {revoke_role}: this function's
        /// purpose is to provide a mechanism for accounts to lose their privileges
        /// if they are compromised (such as when a trusted device is misplaced).
        ///
        /// If the calling account had been granted `role`, emits a {RoleRevoked}
        /// event.
        ///
        /// Requirements:
        ///
        /// - the caller must be `account`.
        #[ink(message)]
        pub fn renounce_role(&mut self, role: RoleId, account: AccountId) {
            access_control::Impl::renounce_role(self, role, account)
        }

        /// Returns whether an id correspond to a registered operation. This
        /// includes both Pending, Ready and Done operations.
        #[ink(message)]
        pub fn is_operation(&self, id: [u8; 32]) -> bool {
            timelock_controller::Impl::is_operation(self, &id)
        }

        /// Returns whether an operation is pending or not.
        #[ink(message)]
        pub fn is_operation_pending(&self, id: [u8; 32]) -> bool {
            timelock_controller::Impl::is_operation_pending(self, &id)
        }

        /// Returns whether an operation is ready or not.
        #[ink(message)]
        pub fn is_operation_ready(&self, id: [u8; 32]) -> bool {
            timelock_controller::Impl::is_operation_ready(self, &id)
        }

        /// Returns whether an operation is done or not.
        #[ink(message)]
        pub fn is_operation_done(&self, id: [u8; 32]) -> bool {
            timelock_controller::Impl::is_operation_done(self, &id)
        }

        /// Returns the timestamp at with an operation becomes ready (0 for
        /// unset operations, 1 for done operations).
        #[ink(message)]
        pub fn get_timestamp(&self, id: [u8; 32]) -> Timestamp {
            timelock_controller::Impl::get_timestamp(self, &id)
        }

        /// Returns the minimum delay for an operation to become valid.
        ///
        /// This value can be changed by executing an operation that calls `updateDelay`.
        #[ink(message)]
        pub fn get_min_delay(&self) -> Timestamp {
            timelock_controller::Impl::get_min_delay(self)
        }

        /// Returns the identifier of an operation containing a single
        /// transaction.
        #[ink(message)]
        pub fn hash_operation(
            &self,
            target: AccountId,
            value: Balance,
            data: Vec<u8>,
            predecessor: Option<[u8; 32]>,
            salt: [u8; 32],
        ) -> [u8; 32] {
            timelock_controller::Impl::hash_operation(
                self,
                &target,
                &value,
                &data,
                &predecessor,
                &salt,
            )
        }

        /// Schedule an operation containing a single transaction.
        ///
        /// Emits a `CallScheduled` event.
        ///
        /// Requirements:
        ///
        /// - the caller must have the 'proposer' role.
        #[ink(message)]
        pub fn schedule(
            &mut self,
            target: AccountId,
            value: Balance,
            data: Vec<u8>,
            predecessor: Option<[u8; 32]>,
            salt: [u8; 32],
            delay: Timestamp,
        ) {
            timelock_controller::Impl::schedule(
                self,
                target,
                value,
                data,
                predecessor,
                salt,
                delay,
            )
        }

        /// Cancel an operation.
        ///
        /// Requirements:
        ///
        /// - the caller must have the 'proposer' role.
        #[ink(message)]
        pub fn cancel(&mut self, id: [u8; 32]) {
            timelock_controller::Impl::cancel(self, id)
        }

        /// Execute an (ready) operation containing a single transaction.
        ///
        /// Emits a `CallExecuted` event.
        ///
        /// Requirements:
        ///
        /// - the caller must have the 'executor' role.
        #[ink(message, payable)]
        pub fn execute(
            &mut self,
            target: AccountId,
            value: Balance,
            data: Vec<u8>,
            predecessor: Option<[u8; 32]>,
            salt: [u8; 32],
        ) {
            timelock_controller::Impl::execute(
                self,
                target,
                value,
                data,
                predecessor,
                salt,
            )
        }
    }
```

In the end, we can add some other messages.

the caller to call need impl the `on_call` message:

```rust
    impl Receiver {
        #[ink(message, payable)]
        pub fn on_call(
            &mut self,
            _operator: AccountId,
            _data: Vec<u8>,
        ) -> bool {
            unimplemented!()
        }
    }
```

like this, NOTE the `on_call` should be payable:

```rust
        #[ink(message, payable)]
        pub fn on_call(
            &mut self,
            operator: AccountId,
            data: Vec<u8>,
        ) -> bool {
            // value to transferred_balance
            let value = Self::env().transferred_balance();

            // emit events
            Self::env().emit_event(CallReceived {
                operator,
                value,
                data,
            });

            // if return false should be error.
            true
        }
```

## Messages for Txs

### schedule

Schedule an operation containing a single transaction.

Emits a `CallScheduled` event.

Requirements:

- the caller must have the 'proposer' role.

```rust
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
```

### cancel

Cancel an operation.

Requirements:

- the caller must have the 'proposer' role.

```rust
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
```

### execute

Execute an (ready) operation containing a single transaction.

Emits a `CallExecuted` event.

Requirements:

- the caller must have the 'executor' role.

```rust
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
```

## Message for Querys

### is_operation

Returns whether an id correspond to a registered operation. This includes both Pending, Ready and Done operations.

```rust
    /// Returns whether an id correspond to a registered operation. This
    /// includes both Pending, Ready and Done operations.
    fn is_operation(&self, id: &[u8; 32]) -> bool {
        self.get_timestamp(id) > E::Timestamp::from(0_u8)
    }
```

### is_operation_pending

Returns whether an operation is pending or not.

```rust
    /// Returns whether an operation is pending or not.
    fn is_operation_pending(&self, id: &[u8; 32]) -> bool {
        self.get_timestamp(id) > E::Timestamp::from(_DONE_TIMESTAMP)
    }
```

### is_operation_ready

Returns whether an operation is ready or not.

```rust
    /// Returns whether an operation is ready or not.
    fn is_operation_ready(&self, id: &[u8; 32]) -> bool {
        let timestamp = self.get_timestamp(id);
        timestamp > E::Timestamp::from(_DONE_TIMESTAMP)
            && timestamp <= Self::block_timestamp()
    }
```

### is_operation_done

Returns whether an operation is done or not.

```rust
    /// Returns whether an operation is done or not.
    fn is_operation_done(&self, id: &[u8; 32]) -> bool {
        self.get_timestamp(id) == E::Timestamp::from(_DONE_TIMESTAMP)
    }
```

### get_timestamp

Returns the timestamp at with an operation becomes ready (0 for unset operations, 1 for done operations).

```rust
    /// Returns the timestamp at with an operation becomes ready (0 for
    /// unset operations, 1 for done operations).
    fn get_timestamp(&self, id: &[u8; 32]) -> E::Timestamp {
        *Storage::<E, Data<E>>::get(self)
            .timestamps
            .get(id)
            .unwrap_or(&E::Timestamp::from(0_u8))
    }
```

### get_min_delay

Returns the minimum delay for an operation to become valid.

This value can be changed by executing an operation that calls `update_delay`.

```rust
    /// Returns the minimum delay for an operation to become valid.
    ///
    /// This value can be changed by executing an operation that calls `update_delay`.
    fn get_min_delay(&self) -> E::Timestamp {
        *Storage::<E, Data<E>>::get(self).min_delay
    }
```

### hash_operation

Returns the identifier of an operation containing a single transaction.

> NOTE: This `hash = Blake2x256(target + value + data + predecessor + salt)`

```rust
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
```

## Apis

### ensure_only_role_or_open_role

To make a function callable only by a certain role. In
addition to checking the sender's role, `AccountId::default()` 's role is also
considered. Granting a role to `AccountId::default()` is equivalent to enabling
this role for everyone.

```rust
    /// To make a function callable only by a certain role. In
    /// addition to checking the sender's role, `address(0)` 's role is also
    /// considered. Granting a role to `address(0)` is equivalent to enabling
    /// this role for everyone.
    fn ensure_only_role_or_open_role(&self, role: RoleId) {
        if !access_control::Impl::has_role(self, role, E::AccountId::default()) {
            access_control::Impl::ensure_caller_role(self, role);
        }
    }
```

## Events

### CallScheduled

Emitted when a call is scheduled as part of operation `id`.

```rust
    /// Emitted when a call is scheduled as part of operation `id`.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct CallScheduled {
        #[ink(topic)]
        pub id: [u8; 32],
        pub target: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
        pub predecessor: Option<[u8; 32]>,
        pub delay: Timestamp,
    }
```

### CallExecuted

Emitted when a call is performed as part of operation `id`.

```rust
    /// Emitted when a call is performed as part of operation `id`.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct CallExecuted {
        #[ink(topic)]
        pub id: [u8; 32],
        pub target: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
    }
```

### Cancelled

Emitted when operation `id` is cancelled.

```rust
    /// Emitted when operation `id` is cancelled.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct Cancelled {
        #[ink(topic)]
        pub id: [u8; 32],
    }
```

### MinDelayChange

Emitted when the minimum delay for future operations is modified.

```rust
    /// Emitted when the minimum delay for future operations is modified.
    #[ink(event)]
    #[metis(timelock_controller)]
    pub struct MinDelayChange {
        pub old_duration: Timestamp,
        pub new_duration: Timestamp,
    }
```
