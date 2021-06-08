pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    lazy::Lazy,
    traits::SpreadLayout,
};

/// The Data of pausable component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data {
    /// is contract current paused
    paused: Lazy<bool>,
}

impl Data {
    pub fn new() -> Self {
        let instance = Self::default();

        instance
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            paused: Lazy::new(false),
        }
    }
}

impl Data {
    /// is_paused is current is paused
    pub fn is_paused(&self) -> bool {
        *self.paused
    }

    /// pause set current paused state to true
    pub fn pause(&mut self) {
        Lazy::set(&mut self.paused, true);
    }

    /// unpause set current paused state to false
    pub fn unpause(&mut self) {
        Lazy::set(&mut self.paused, false);
    }
}
