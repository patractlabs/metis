use super::arithmetic::AtLeast32BitUnsigned;
use ink_env::Clear;
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};

#[cfg(feature = "std")]
pub trait EnvAccountId:
    'static
    + scale::Codec
    + Clone
    + PartialEq
    + Eq
    + Ord
    + std::fmt::Debug
    + Into<ink_env::AccountId>
{
}

#[cfg(feature = "std")]
impl<T> EnvAccountId for T where
    T: 'static
        + scale::Codec
        + Clone
        + PartialEq
        + Eq
        + Ord
        + std::fmt::Debug
        + Into<ink_env::AccountId>
{
}

#[cfg(not(feature = "std"))]
pub trait EnvAccountId:
    'static + scale::Codec + Clone + PartialEq + Eq + Ord + Into<ink_env::AccountId>
{
}

#[cfg(not(feature = "std"))]
impl<T> EnvAccountId for T where
    T: 'static + scale::Codec + Clone + PartialEq + Eq + Ord + Into<ink_env::AccountId>
{
}

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
    + Into<u128>
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
        + Into<u128>
{
}

#[cfg(feature = "std")]
pub trait Balance:
    'static
    + scale::Codec
    + std::fmt::Debug
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
    + Into<u128>
{
}

#[cfg(feature = "std")]
impl<T> Balance for T where
    T: 'static
        + scale::Codec
        + std::fmt::Debug
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
        + Into<u128>
{
}

#[cfg(feature = "std")]
pub trait Hash:
    'static
    + scale::Codec
    + std::fmt::Debug
    + ::scale_info::TypeInfo
    + ::ink_storage::traits::StorageLayout
    + SpreadLayout
    + PackedLayout
    + Copy
    + Clone
    + Clear
    + PartialEq
    + Eq
    + Ord
    + AsRef<[u8]>
    + AsMut<[u8]>
    + Default
{
}

#[cfg(feature = "std")]
impl<T> Hash for T where
    T: 'static
        + scale::Codec
        + std::fmt::Debug
        + ::scale_info::TypeInfo
        + ::ink_storage::traits::StorageLayout
        + SpreadLayout
        + PackedLayout
        + Copy
        + Clone
        + Clear
        + PartialEq
        + Eq
        + Ord
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Default
{
}

#[cfg(not(feature = "std"))]
pub trait Hash:
    'static
    + scale::Codec
    + Copy
    + Clone
    + SpreadLayout
    + PackedLayout
    + Clear
    + PartialEq
    + Eq
    + Ord
    + AsRef<[u8]>
    + AsMut<[u8]>
    + Default
{
}

#[cfg(not(feature = "std"))]
impl<T> Hash for T where
    T: 'static
        + scale::Codec
        + Copy
        + Clone
        + SpreadLayout
        + PackedLayout
        + Clear
        + PartialEq
        + Eq
        + Ord
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Default
{
}

/// The type of timestamps.
#[cfg(feature = "std")]
pub trait Timestamp:
    'static
    + scale::Codec
    + Copy
    + ::scale_info::TypeInfo
    + ::ink_storage::traits::StorageLayout
    + SpreadLayout
    + PackedLayout
    + Clone
    + PartialEq
    + Eq
    + AtLeast32BitUnsigned
{
}

#[cfg(feature = "std")]
impl<T> Timestamp for T where
    T: 'static
        + scale::Codec
        + Copy
        + ::scale_info::TypeInfo
        + ::ink_storage::traits::StorageLayout
        + SpreadLayout
        + PackedLayout
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
{
}

#[cfg(not(feature = "std"))]
pub trait Timestamp:
    'static
    + scale::Codec
    + Copy
    + SpreadLayout
    + PackedLayout
    + Clone
    + PartialEq
    + Eq
    + AtLeast32BitUnsigned
{
}

#[cfg(not(feature = "std"))]
impl<T> Timestamp for T where
    T: 'static
        + scale::Codec
        + Copy
        + SpreadLayout
        + PackedLayout
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
{
}

/// The type of block number.

#[cfg(feature = "std")]
pub trait BlockNumber:
    'static
    + scale::Codec
    + ::scale_info::TypeInfo
    + ::ink_storage::traits::StorageLayout
    + Copy
    + Clone
    + PartialEq
    + Eq
    + AtLeast32BitUnsigned
{
}

#[cfg(feature = "std")]
impl<T> BlockNumber for T where
    T: 'static
        + scale::Codec
        + ::scale_info::TypeInfo
        + ::ink_storage::traits::StorageLayout
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned
{
}

#[cfg(not(feature = "std"))]
pub trait BlockNumber:
    'static + scale::Codec + Copy + Clone + PartialEq + Eq + AtLeast32BitUnsigned
{
}

#[cfg(not(feature = "std"))]
impl<T> BlockNumber for T where
    T: 'static + scale::Codec + Copy + Clone + PartialEq + Eq + AtLeast32BitUnsigned
{
}

pub trait ChainExtension {}
