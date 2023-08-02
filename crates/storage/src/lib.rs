#![cfg_attr(not(feature = "std"), no_std)]

use ink::primitives::AccountId;

use ink::env::DefaultEnvironment;

pub type Balance = u128;

use rmrk_traits::psp34::PSP34;

#[derive(Debug)]
#[ink::storage_item]
pub struct RmrkStorage {
    pub psp34: ink::contract_ref!(PSP34, DefaultEnvironment),
    pub minting: RmrkMinting,
}

#[derive(Default, Debug)]
#[ink::storage_item]
pub struct RmrkMinting {
    pub max_supply: Option<u64>,
    pub price_per_mint: Balance,
}

impl RmrkStorage {
    pub fn new(psp34: AccountId) -> Self {
        Self {
            psp34: psp34.into(),
            minting: RmrkMinting::default(),
        }
    }
}

pub trait RmrkStorageSelector {
    fn storage(&mut self) -> &mut RmrkStorage;
}

#[macro_export]
macro_rules! impl_rmrk_storage {
    ($contract:ident) => {
        impl ::rmrk::rmrk_storage::RmrkStorageSelector for $contract {
            fn storage(&mut self) -> &mut ::rmrk::rmrk_storage::RmrkStorage {
                &mut self.rmrk
            }
        }
    };
}

