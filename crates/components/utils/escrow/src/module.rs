pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::{
        hashmap::Entry,
        HashMap as StorageHashMap,
    },
    traits::SpreadLayout,
};

/// The Data of escrow component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
    /// The owner of contract
    pub deposits: StorageHashMap<E::AccountId, E::Balance>,
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
            deposits: StorageHashMap::default(),
        }
    }
}

impl<E> Data<E>
where
    E: Env,
{
    pub fn get(&self, payee: &E::AccountId) -> E::Balance {
        self.deposits
            .get(payee)
            .copied()
            .unwrap_or(E::Balance::from(0_u8))
    }

    pub fn add(&mut self, payee: &E::AccountId, amt: &E::Balance) {
        match self.deposits.entry(payee.clone()) {
            Entry::Vacant(vacant) => {
                vacant.insert(amt.clone());
            }
            Entry::Occupied(mut occupied) => {
                occupied.insert(occupied.get().clone() + amt.clone());
            }
        };
    }

    pub fn clean(&mut self, payee: &E::AccountId) {
        match self.deposits.entry(payee.clone()) {
            Entry::Vacant(_) => {
                panic!("Escrow: no found deposit");
            }
            Entry::Occupied(mut occupied) => {
                occupied.insert(E::Balance::from(0_u8));
            }
        };
    }
}
