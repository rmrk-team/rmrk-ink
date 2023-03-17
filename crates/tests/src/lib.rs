#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod helpers;
mod minting;

// #[ink::contract]
// pub mod contract {

//     #[ink(storage)]
//     pub struct Noop;

//     impl Noop {
//         #[ink(constructor)]
//         pub fn new() -> Self {
//             Noop {}
//         }

//         #[ink(message)]
//         pub fn noop(&mut self) {}
//     }
// }
