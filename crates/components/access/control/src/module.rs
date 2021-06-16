pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};

use crate::types::RoleId;

/// The Data of ownership component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
    /// the account - role relationship map
    pub roles: StorageHashMap<(E::AccountId, RoleId), ()>,

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

impl<E> Data<E> where E: Env {}
