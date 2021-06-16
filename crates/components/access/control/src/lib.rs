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
    /// Emit RoleAdminChanged event
    fn emit_event_role_admin_changed(
        &mut self,
        role: RoleId,
        previous_admin_role: RoleId,
        new_admin_role: RoleId,
    );
}

/// The `Impl` define component impl funcs
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {
    }

    /// Return the owner AccountId
    fn has_role(&self, _role: RoleId, _account: E::AccountId) -> bool {
        false
    }

    /// Panic if `owner` is not an owner
    fn ensure_role(&self, _role: RoleId, _owner: &E::AccountId) {
    }

    /// Panic if caller is not granted role
    fn ensure_caller_role(&self, role: RoleId) {
        self.ensure_role(role, &Self::caller());
    }
}

impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
