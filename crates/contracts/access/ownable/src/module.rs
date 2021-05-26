
pub use metis_contract::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{lazy::Lazy, traits::SpreadLayout};

#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    owner: Lazy<Option<E::AccountId>>,
}

pub trait Storage<E: Env> {
    fn get(&self) -> &Data<E>;
    fn get_mut(&mut self) -> &mut Data<E>;
}

impl<E: Env> Data<E> {
    pub fn new() -> Self {
        Self {
            owner: Lazy::default(),
        }
    }
}

impl<E: Env> Data<E> {
    pub fn get_ownership(&self) -> &Option<E::AccountId> {
        &self.owner
    }

    pub fn set_ownership(&mut self, owner: &Option<E::AccountId>) {
        Lazy::set(&mut self.owner, owner.clone());
    }
}
