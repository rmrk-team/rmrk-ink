#![cfg_attr(not(feature = "std"), no_std)]

pub mod config;
pub mod query;

pub use rmrk_common::{
    counter,
    errors,
    roles,
    types,
    utils,
};

pub mod storage {
    pub use rmrk_catalog::*;
    pub use rmrk_equippable::*;
    pub use rmrk_minting::*;
    pub use rmrk_multiasset::*;
    pub use rmrk_nesting::*;
}

pub mod traits {
    pub use rmrk_catalog::traits::*;
    pub use rmrk_equippable::traits::*;
    pub use rmrk_minting::traits::*;
    pub use rmrk_multiasset::traits::*;
    pub use rmrk_nesting::traits::*;
}

pub mod extensions {
    pub use rmrk_catalog::extensions::autoindex::*;
    pub use rmrk_minting::extensions::autoindex::*;
    pub use rmrk_multiasset::extensions::autoindex::*;
}
