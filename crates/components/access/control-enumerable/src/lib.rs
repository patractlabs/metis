//! Extension of `Access Control` Component that allows enumerating the members of each role.

pub use access_control::{
    Error,
    Result,
    RoleId,
};
use metis_access_control as access_control;

use metis_lang::{
    Env,
    Storage,
};

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::{
        hashmap::Entry,
        HashMap as StorageHashMap,
    },
    traits::SpreadLayout,
};

/// The Data of ERC20 component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    role_members: StorageHashMap<RoleId, Vec<E::AccountId>>,
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
            role_members: StorageHashMap::default(),
        }
    }
}

impl<E: Env> Data<E> {
    fn add_member(&mut self, role: &RoleId, member: &E::AccountId) {
        self.role_members
            .entry(role.clone())
            .or_insert(Vec::default())
            .push(member.clone());
    }

    fn remove_member(&mut self, role: &RoleId, member: &E::AccountId) {
        let mut occupied = match self.role_members.entry(role.clone()) {
            Entry::Vacant(_) => panic!("no found role by id"),
            Entry::Occupied(occupied) => occupied,
        };

        let members = occupied.get_mut();

        for i in (0..members.len()).rev() {
            if members[i] == *member {
                members.swap_remove(i);
                return // the vec will have just one member
            }
        }
    }
}

/// Extension of {ERC20} that adds a cap to the supply of tokens.
pub trait Impl<E>: access_control::Impl<E> + Storage<E, Data<E>>
where
    E: Env,
{
    /// Returns one of the accounts that have `role`. `index` must be a
    /// value between 0 and {getRoleMemberCount}, non-inclusive.
    ///
    /// Role bearers are not sorted in any particular way, and their ordering may
    /// change at any point.
    ///
    /// WARNING: When using {getRoleMember} and {getRoleMemberCount}, make sure
    /// you perform all queries on the same block. See the following
    /// https://forum.openzeppelin.com/t/iterating-over-elements-on-enumerableset-in-openzeppelin-contracts/2296[forum post]
    /// for more information.
    fn get_role_member(&self, role: &RoleId, index: usize) -> E::AccountId {
        match Storage::<E, Data<E>>::get(self).role_members.get(role) {
            None => panic!("no found role by id"),
            Some(members) => members[index].clone(), // will panic when out of index
        }
    }

    /// Returns the number of accounts that have `role`. Can be used
    /// together with {getRoleMember} to enumerate all bearers of a role.
    fn get_role_member_count(&self, role: &RoleId) -> usize {
        match Storage::<E, Data<E>>::get(self).role_members.get(role) {
            None => panic!("no found role by id"),
            Some(members) => members.len(),
        }
    }

    /// Returns `true` if `account` has been granted `role`.
    fn has_role(&self, role: RoleId, account: E::AccountId) -> bool {
        access_control::Impl::has_role(self, role, account)
    }

    /// @dev Returns the admin role that controls `role`. See {grantRole} and
    /// {revokeRole}.
    ///
    /// To change a role's admin, use {_setRoleAdmin}.
    fn get_role_admin(&self, role: RoleId) -> Option<RoleId> {
        access_control::Impl::get_role_admin(self, role)
    }

    /// Panic if `owner` is not an owner
    fn ensure_role(&self, role: RoleId, account: E::AccountId) {
        access_control::Impl::ensure_role(self, role, account)
    }

    /// Panic if caller is not granted role
    fn ensure_caller_role(&self, role: RoleId) {
        access_control::Impl::ensure_caller_role(self, role)
    }

    /// Panic error if `account` is missing the admin role of the `role`.
    fn ensure_admin_role(&self, role: RoleId, account: E::AccountId) {
        access_control::Impl::ensure_admin_role(self, role, account)
    }

    /// Return error if `account` is missing `role`.
    fn check_role(&self, role: RoleId, account: E::AccountId) -> Result<()> {
        access_control::Impl::check_role(self, role, account)
    }

    /// Return error if `account` is missing the admin role of the `role`.
    fn check_admin_role(&self, role: RoleId, account: E::AccountId) -> Result<()> {
        access_control::Impl::check_admin_role(self, role, account)
    }

    /// Sets `adminRole` as ``role``'s admin role.
    ///
    /// Emits a {RoleAdminChanged} event.
    fn _set_role_admin(&mut self, role: RoleId, admin_role: RoleId) {
        access_control::Impl::_set_role_admin(self, role, admin_role)
    }

    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn grant_role(&mut self, role: RoleId, account: E::AccountId) {
        // if has a role, the add will revert by panic
        // grant_role will call the _setup_role but not add
        Storage::<E, Data<E>>::get_mut(self).add_member(&role, &account);

        access_control::Impl::grant_role(self, role, account)
    }

    /// @dev Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    fn revoke_role(&mut self, role: RoleId, account: E::AccountId) {
        Storage::<E, Data<E>>::get_mut(self).remove_member(&role, &account);

        access_control::Impl::revoke_role(self, role, account)
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
    fn renounce_role(&mut self, role: RoleId, account: E::AccountId) {
        Storage::<E, Data<E>>::get_mut(self).remove_member(&role, &account);

        access_control::Impl::renounce_role(self, role, account)
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
    fn _setup_role(&mut self, role: RoleId, account: E::AccountId) {
        // if has a role, the add will revert by panic
        Storage::<E, Data<E>>::get_mut(self).add_member(&role, &account);

        access_control::Impl::_setup_role(self, role, account)
    }
}
