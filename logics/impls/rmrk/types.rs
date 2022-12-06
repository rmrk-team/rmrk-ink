//! Types definition for RMRK contract
use ink_prelude::vec::Vec;
use ink_storage::Mapping;
use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        Balance,
        String,
    },
};

pub const STORAGE_NESTING_KEY: u32 = openbrush::storage_unique_key!(NestingData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_NESTING_KEY)]
pub struct NestingData {
    pub pending_children: Mapping<Id, Vec<ChildNft>>,
    pub accepted_children: Mapping<Id, Vec<ChildNft>>,
}

// Collection id is the address of child contract
pub type CollectionId = AccountId;

// Nft is a tuple of collection and TokenId and refers to the Child nft
pub type ChildNft = (CollectionId, Id);

pub type BaseId = u32;
pub type SlotId = u32;
pub type PartId = u32;
pub type AssetId = u32;

pub const STORAGE_PSP34_KEY: u32 = openbrush::storage_unique_key!(Psp34CustomData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_PSP34_KEY)]
pub struct Psp34CustomData {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}

pub const STORAGE_MUSLTIASSET_KEY: u32 = openbrush::storage_unique_key!(MultiAssetData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MUSLTIASSET_KEY)]
pub struct MultiAssetData {
    /// List of available asset entries for this collection
    pub collection_asset_entries: Vec<String>,

    /// Mapping of tokenId to an array of active assets
    pub accepted_assets: Mapping<Id, Vec<AssetId>>,

    /// Mapping of tokenId to an array of pending assets
    pub pending_assets: Mapping<Id, Vec<AssetId>>,
}
