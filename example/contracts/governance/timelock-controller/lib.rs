#![cfg_attr(not(feature = "std"), no_std)]

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

    #[cfg(not(feature = "ink-as-dependency"))]
    impl timelock_controller::Impl<TimelockController> for TimelockController {}

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

    // impl
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
}
