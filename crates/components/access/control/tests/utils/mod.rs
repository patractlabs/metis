#![cfg_attr(not(feature = "std"), no_std)]

mod event;

pub use event::{
    assert_emitted_event_len,
    encoded_into_hash,
    PrefixedValue,
};