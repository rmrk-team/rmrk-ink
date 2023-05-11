#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod catalog;
pub mod internal;
pub mod traits;
pub mod extensions {
    pub mod autoindex;
}

pub use catalog::*;
pub use internal::*;
pub use traits::*;
