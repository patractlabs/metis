//! Contract module that allows children to implement role-based access
//! control mechanisms. This is a lightweight version that doesn't allow enumerating role
//! members except through off-chain means by accessing the contract event logs. Some
//! applications may benefit from on-chain enumerability, for those cases see
//! `access-control-eunmerable`.
//!
//! Roles are referred to by their `bytes32` identifier. These should be exposed
//! in the external API and be unique. The best way to achieve this is by
//! using `public constant` hash digests:
//!
//! bytes32 public constant MY_ROLE = keccak256("MY_ROLE");
//!
//! Roles can be used to represent a set of permissions. To restrict access to a
//! function call, use {hasRole}:
//!
//! function foo() public {
//!     require(hasRole(MY_ROLE, msg.sender));
//!     ...
//! }
//!
//! Roles can be granted and revoked dynamically via the {grantRole} and
//! {revokeRole} functions. Each role has an associated admin role, and only
//! accounts that have a role's admin role can call {grantRole} and {revokeRole}.
//!
//! By default, the admin role for all roles is `DEFAULT_ADMIN_ROLE`, which means
//! that only accounts with this role will be able to grant or revoke other
//! roles. More complex role relationships can be created by using
//! {_setRoleAdmin}.
//!
//! WARNING: The `DEFAULT_ADMIN_ROLE` is also its own admin: it has permission to
//! grant and revoke this role. Extra precautions should be taken to secure
//! accounts that have been granted it.

#![cfg_attr(not(feature = "std"), no_std)]

mod module;
mod types;

use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

pub use module::Data;

pub use types::{
    Error,
    Result,
    RoleId,
};

/// The `EventEmit` impl the event emit api for component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emitted when `newAdminRole` is set as ``role``'s admin role, replacing `previousAdminRole`
    ///
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// {RoleAdminChanged} not being emitted signaling this.
    fn emit_event_role_admin_changed(
        &mut self,
        role: RoleId,
        previous_admin_role: Option<RoleId>,
        new_admin_role: RoleId,
    );

    /// Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call, an admin role
    /// bearer except when using {_setupRole}.
    fn emit_event_role_granted(
        &mut self,
        role: RoleId,
        account: E::AccountId,
        sender: E::AccountId,
    );

    /// Emitted when `account` is revoked `role`.
    ///
    /// `sender` is the account that originated the contract call:
    ///   - if using `revokeRole`, it is the admin role bearer
    ///   - if using `renounceRole`, it is the role bearer (i.e. `account`)
    fn emit_event_role_revoked(
        &mut self,
        role: RoleId,
        account: E::AccountId,
        sender: E::AccountId,
    );
}

/// The `Impl` define component impl funcs
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// Returns `true` if `account` has been granted `role`.
    fn has_role(&self, role: RoleId, account: E::AccountId) -> bool {
        self.get().has_role(role, account)
    }

    /// @dev Returns the admin role that controls `role`. See {grantRole} and
    /// {revokeRole}.
    ///
    /// To change a role's admin, use {_setRoleAdmin}.
    fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
        self.get().get_role_admin(role)
    }

    /// Panic if `owner` is not an owner
    fn ensure_role(&self, role: RoleId, account: E::AccountId) {
        assert!(
            self.has_role(role, account),
            "AccessControl: account is missing role"
        );
    }

    /// Panic if caller is not granted role
    fn ensure_caller_role(&self, role: RoleId) {
        self.ensure_role(role, Self::caller());
    }

    /// Return error if `account` is missing the admin role of the `role`.
    fn check_admin_role(&self, role: RoleId, account: E::AccountId) -> Result<()> {
        match self.get_role_admin(role) {
            Some(admin_role) => self.check_role(admin_role, account),
            None => Err(Error::AdminRoleNotFound),
        }
    }

    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn grant_role(&mut self, role: RoleId, account: E::AccountId) -> Result<()> {
        // check the admin role
        self.check_admin_role(role, Self::caller())?;

        self._setup_role(role, account)
    }

    /// @dev Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn revoke_role(&mut self, role: RoleId, account: E::AccountId) -> Result<()> {
        let caller = Self::caller();

        // check the admin role
        self.check_admin_role(role, caller.clone())?;

        // if has not role, so return err
        self.get_mut().revoke_role(role, account.clone())?;

        // emit if revoke role success
        self.emit_event_role_revoked(role, account, caller);

        Ok(())
    }

    /// @dev Revokes `role` from the calling account.
    ///
    /// Roles are often managed via {grantRole} and {revokeRole}: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// If the calling account had been granted `role`, emits a {RoleRevoked}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must be `account`.
    fn renounce_role(&mut self, role: RoleId, account: E::AccountId) -> Result<()> {
        let caller = Self::caller();

        // check the caller is account
        if caller != account {
            return Err(Error::AcccountIsNotCaller)
        }

        // if has not role, so return err
        self.get_mut().revoke_role(role, account.clone())?;

        // emit if revoke role success
        self.emit_event_role_revoked(role, account, caller);

        Ok(())
    }

    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event. Note that unlike {grant_role}, this function doesn't perform any
    /// checks on the calling account.
    ///
    /// [WARNING]
    /// ====
    /// This function should only be called from the constructor when setting
    /// up the initial roles for the system.
    ///
    /// Using this function in any other way is effectively circumventing the admin
    /// system imposed by {AccessControl}.
    /// ====
    fn _setup_role(&mut self, role: RoleId, account: E::AccountId) -> Result<()> {
        let caller = Self::caller();

        // if has role, so return error
        self.get_mut().grant_role(role, account.clone())?;

        // emit if grant role success
        self.emit_event_role_granted(role, account, caller);

        Ok(())
    }

    fn _set_role_admin(&mut self, role: RoleId, admin_role: RoleId){
        let old_admin_role = self.get_role_admin(role);

        self.get_mut().set_role_admin(role, admin_role);

        self.emit_event_role_admin_changed(role, old_admin_role, admin_role);
    }

    /// Return error if `account` is missing `role`.
    fn check_role(&self, role: RoleId, account: E::AccountId) -> Result<()> {
        if self.has_role(role, account) {
            Ok(())
        } else {
            Err(Error::RoleNotFound)
        }
    }
}

impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
