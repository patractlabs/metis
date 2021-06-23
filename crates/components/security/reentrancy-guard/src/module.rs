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
#[derive(Debug)]
pub struct Data {
    /// is contract current paused
    status: Lazy<u8>,

    key: ink_primitives::Key,
}

impl ink_storage::traits::SpreadLayout for Data {
    const FOOTPRINT: u64 = <Lazy<u8> as SpreadLayout>::FOOTPRINT;

    fn pull_spread(ptr: &mut ink_storage::traits::KeyPtr) -> Self {
        Self {
            status: <Lazy<u8> as SpreadLayout>::pull_spread(ptr),
            key: ptr.key().clone(),
        }
    }

    fn push_spread(&self, ptr: &mut ink_storage::traits::KeyPtr) {
        <Lazy<u8> as SpreadLayout>::push_spread(&self.status, ptr)
    }

    fn clear_spread(&self, ptr: &mut ink_storage::traits::KeyPtr) {
        <Lazy<u8> as SpreadLayout>::clear_spread(&self.status, ptr)
    }
}

impl Data {
    fn flush(&self) {
        ink_storage::traits::push_spread_root::<Data>(self, &self.key);
    }
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
            key: ink_primitives::Key::default(),
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
        Lazy::set(&mut self.status, _ENTERED);
        self.flush()
    }

    /// set current status to not entered
    pub fn set_not_entered(&mut self) {
        Lazy::set(&mut self.status, _NOT_ENTERED);
        self.flush()
    }
}
