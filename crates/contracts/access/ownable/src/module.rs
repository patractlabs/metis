pub use metis_contract::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{lazy::Lazy, traits::SpreadLayout};

#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
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
    pub fn get_ownership(&self) -> &Option<E::AccountId> {
        &self.owner
    }

    pub fn set_ownership(&mut self, owner: &Option<E::AccountId>) {
        Lazy::set(&mut self.owner, owner.clone());
    }
}
