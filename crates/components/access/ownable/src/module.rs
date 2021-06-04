pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{lazy::Lazy, traits::SpreadLayout};

/// The Data of ownership component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
    /// The owner of contract
    owner: Lazy<Option<E::AccountId>>,
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
            owner: Lazy::default(),
        }
    }
}

impl<E> Data<E>
where
    E: Env,
{
    /// get_ownership get owner for contract
    pub fn get_ownership(&self) -> &Option<E::AccountId> {
        &self.owner
    }

    /// set_ownership set owner for contract
    pub fn set_ownership(&mut self, owner: &Option<E::AccountId>) {
        Lazy::set(&mut self.owner, owner.clone());
    }
}
