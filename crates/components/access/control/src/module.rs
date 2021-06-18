pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::{
        hashmap::Entry,
        HashMap as StorageHashMap,
    },
    traits::SpreadLayout,
};

use crate::types::{
    Error,
    Result,
    RoleId,
};

/// The Data of ownership component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
    /// the account - role relationship map
    pub roles: StorageHashMap<(RoleId, E::AccountId), ()>,

    /// the admin role of a role
    pub admin_roles: StorageHashMap<RoleId, RoleId>,
}

impl<E> Data<E>
where
    E: Env,
{
    pub fn new() -> Self {
        let instance = Self::default();

        instance
    }
}

impl<E> Default for Data<E>
where
    E: Env,
{
    fn default() -> Self {
        Self {
            roles: StorageHashMap::default(),
            admin_roles: StorageHashMap::default(),
        }
    }
}

impl<E> Data<E>
where
    E: Env,
{
    pub fn has_role(&self, role: RoleId, account: E::AccountId) -> bool {
        self.roles.contains_key(&(role, account))
    }

    pub fn grant_role(&mut self, role: RoleId, account: E::AccountId) -> Result<()> {
        let key = (role, account);

        if self.roles.contains_key(&key) {
            return Err(Error::AccountRoleExists)
        }

        self.roles.insert(key, ());

        Ok(())
    }

    pub fn revoke_role(&mut self, role: RoleId, account: E::AccountId) -> Result<()> {
        let occupied = match self.roles.entry((role, account)) {
            Entry::Vacant(_) => return Err(Error::NotHasRole),
            Entry::Occupied(occupied) => occupied,
        };

        occupied.remove_entry();

        Ok(())
    }
}
