#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[cfg(feature = "e2e-tests")]
mod helpers;

#[cfg(feature = "e2e-tests")]
mod minting;
