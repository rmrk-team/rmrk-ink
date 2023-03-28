#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub use rmrk_common::{
    errors,
    roles,
    types,
    utils,
};

pub mod catalog;
pub mod internal;
pub mod traits;
