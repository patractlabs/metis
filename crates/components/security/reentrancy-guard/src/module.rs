pub use metis_lang::Env;

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    lazy::Lazy,
    traits::SpreadLayout,
};

const _NOT_ENTERED: u8 = 1;
const _ENTERED: u8 = 2;

/// The Data of pausable component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data {
    /// is contract current paused
    status: Lazy<u8>,
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            status: Lazy::new(_NOT_ENTERED),
        }
    }
}

impl Data {
    /// is_entered is current is paused
    pub fn is_entered(&self) -> bool {
        *self.status == _ENTERED
    }

    /// set current status to entered
    pub fn set_entered(&mut self) {
        Lazy::set(&mut self.status, _ENTERED)
    }

     /// set current status to not entered
    pub fn set_not_entered(&mut self) {
        Lazy::set(&mut self.status, _NOT_ENTERED)
    }
}
