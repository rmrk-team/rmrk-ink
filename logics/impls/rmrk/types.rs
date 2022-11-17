use ink_prelude::collections::BTreeMap;
use openbrush::{contracts::psp34::Id, traits::AccountId};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct NestingData {
    pub pending_children: BTreeMap<ItemId, Nft>,
    pub accepted_children: BTreeMap<ItemId, Nft>,
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
pub type Nft = (CollectionId, Id);

// ItemId is member of this collection
pub type ItemId = Id;
