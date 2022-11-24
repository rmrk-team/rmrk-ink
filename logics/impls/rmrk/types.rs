//! Types definition for RMRK contract
//!
use ink_prelude::collections::{BTreeMap, BTreeSet};
use openbrush::traits::Balance;
use openbrush::{contracts::psp34::Id, traits::AccountId};

pub const STORAGE_NESTING_KEY: u32 = openbrush::storage_unique_key!(NestingData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_NESTING_KEY)]
pub struct NestingData {
    pub pending_children: BTreeMap<Id, BTreeSet<ChildNft>>,
    pub accepted_children: BTreeMap<Id, BTreeSet<ChildNft>>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ChildStatus {
    Pending,
    Accepted,
}

// Collection id is the address of child contract
pub type CollectionId = AccountId;

// Nft is a tuple of collection and TokenId and refers to the Child nft
pub type ChildNft = (CollectionId, Id);

pub const STORAGE_PSP34_KEY: u32 = openbrush::storage_unique_key!(Psp34CustomData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_PSP34_KEY)]
pub struct Psp34CustomData {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}
