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

pub use types::RoleId;

/// The `EventEmit` impl the event emit api for component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emitted when `newAdminRole` is set as ``role``'s admin role, replacing `previousAdminRole`
    ///
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// {RoleAdminChanged} not being emitted signaling this.
    ///
    /// _Available since v3.1._
    fn emit_event_role_admin_changed(
        &mut self,
        role: RoleId,
        previous_admin_role: RoleId,
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
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {}

    /// Returns `true` if `account` has been granted `role`.
    fn has_role(&self, _role: RoleId, _account: E::AccountId) -> bool {
        false
    }

    /// @dev Returns the admin role that controls `role`. See {grantRole} and
    /// {revokeRole}.
    ///
    /// To change a role's admin, use {_setRoleAdmin}.
    fn get_role_admin(&self, _role: RoleId) -> RoleId {
        RoleId::default()
    }

    /// Panic if `owner` is not an owner
    fn ensure_role(&self, _role: RoleId, _owner: &E::AccountId) {}

    /// Panic if caller is not granted role
    fn ensure_caller_role(&self, role: RoleId) {
        self.ensure_role(role, &Self::caller());
    }

    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn grant_role(&mut self, _role: RoleId, _account: E::AccountId) {}

    /// @dev Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn revoke_role(&mut self, _role: RoleId, _account: E::AccountId) {}

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
    fn renounce_role(&mut self, _role: RoleId, _account: E::AccountId) {}

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
    fn _setup_role(&mut self, _role: RoleId, _account: E::AccountId) {}
}

impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
