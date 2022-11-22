//! Types definition for RMRK contract
//! 
use ink_prelude::collections::{BTreeMap, BTreeSet};
use openbrush::{contracts::psp34::Id, traits::AccountId};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
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

