use super::arithmetic::AtLeast32BitUnsigned;
use ink_env::Clear;
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};

#[cfg(feature = "std")]
pub trait EnvAccountId: 'static + scale::Codec + Clone + PartialEq + Eq + Ord + std::fmt::Debug {}

#[cfg(feature = "std")]
impl<T> EnvAccountId for T where T: 'static + scale::Codec + Clone + PartialEq + Eq + Ord + std::fmt::Debug {}

#[cfg(not(feature = "std"))]
pub trait EnvAccountId: 'static + scale::Codec + Clone + PartialEq + Eq + Ord {}

#[cfg(not(feature = "std"))]
impl<T> EnvAccountId for T where T: 'static + scale::Codec + Clone + PartialEq + Eq + Ord {}


#[cfg(not(feature = "std"))]
pub trait AccountId: EnvAccountId + Default + SpreadLayout + PackedLayout {}

#[cfg(not(feature = "std"))]
impl<T> AccountId for T where T: EnvAccountId + Default + SpreadLayout + PackedLayout {}

#[cfg(feature = "std")]
pub trait AccountId:
    EnvAccountId
    + Default
    + ::scale_info::TypeInfo
    + ::ink_storage::traits::StorageLayout
    + SpreadLayout
    + PackedLayout
{
}

#[cfg(feature = "std")]
impl<T> AccountId for T where
    T: EnvAccountId
        + Default
        + ::scale_info::TypeInfo
        + ::ink_storage::traits::StorageLayout
        + SpreadLayout
        + PackedLayout
{
}

#[cfg(not(feature = "std"))]
pub trait Balance:
    'static
    + scale::Codec
    + Copy
    + Clone
    + PartialEq
    + Eq
    + AtLeast32BitUnsigned
    + Default
    + SpreadLayout
    + PackedLayout
{
}

#[cfg(not(feature = "std"))]
impl<T> Balance for T where
    T: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
        + Default
        + SpreadLayout
        + PackedLayout
{
}

#[cfg(feature = "std")]
pub trait Balance:
    'static
    + scale::Codec
    + Copy
    + Clone
    + PartialEq
    + Eq
    + AtLeast32BitUnsigned
    + Default
    + ::scale_info::TypeInfo
    + ::ink_storage::traits::StorageLayout
    + SpreadLayout
    + PackedLayout
{
}

#[cfg(feature = "std")]
impl<T> Balance for T where
    T: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
        + Default
        + ::scale_info::TypeInfo
        + ::ink_storage::traits::StorageLayout
        + SpreadLayout
        + PackedLayout
{
}

pub trait Hash:
    'static
    + scale::Codec
    + Copy
    + Clone
    + Clear
    + PartialEq
    + Eq
    + Ord
    + AsRef<[u8]>
    + AsMut<[u8]>
{
}

impl<T> Hash for T where
    T: 'static
        + scale::Codec
        + Copy
        + Clone
        + Clear
        + PartialEq
        + Eq
        + Ord
        + AsRef<[u8]>
        + AsMut<[u8]>
{
}

/// The type of timestamps.
pub trait Timestamp:
    'static + scale::Codec + Copy + Clone + PartialEq + Eq + AtLeast32BitUnsigned
{
}

impl<T> Timestamp for T where
    T: 'static + scale::Codec + Copy + Clone + PartialEq + Eq + AtLeast32BitUnsigned
{
}

/// The type of block number.
pub trait BlockNumber:
    'static + scale::Codec + Copy + Clone + PartialEq + Eq + AtLeast32BitUnsigned
{
}

impl<T> BlockNumber for T where
    T: 'static + scale::Codec + Copy + Clone + PartialEq + Eq + AtLeast32BitUnsigned
{
}

pub trait ChainExtension {}
