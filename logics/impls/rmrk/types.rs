//! Types definition for RMRK contract

use ink_prelude::vec::Vec;
use ink_primitives::{
    Key,
    KeyPtr,
};
use ink_storage::{
    traits::{
        ExtKeyPtr,
        PackedAllocate,
        PackedLayout,
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};
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
pub type EquippableGroupId = u32;

pub const STORAGE_PSP34_KEY: u32 = openbrush::storage_unique_key!(UtilsData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_PSP34_KEY)]
pub struct UtilsData {
    pub collection_id: u32,
}

pub const STORAGE_MINTING_KEY: u32 = openbrush::storage_unique_key!(MintingData);
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MINTING_KEY)]
pub struct MintingData {
    pub last_token_id: u64,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}

pub const STORAGE_MUSLTIASSET_KEY: u32 = openbrush::storage_unique_key!(MultiAssetData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MUSLTIASSET_KEY)]
pub struct MultiAssetData {
    /// List of available asset entries for this collection
    pub collection_asset_entries: Vec<Asset>,

    /// Mapping of tokenId to an array of active assets
    pub accepted_assets: Mapping<Id, Vec<AssetId>>,

    /// Mapping of tokenId to an array of pending assets
    pub pending_assets: Mapping<Id, Vec<AssetId>>,
}

/// Part's details
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct Asset {
    pub asset_id: AssetId,
    pub equippable_group_id: EquippableGroupId,
    pub asset_uri: String,
}

impl ink_storage::traits::PackedAllocate for Asset {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut *self, at)
    }
}

impl SpreadAllocate for Asset {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.next_for::<Asset>();
        Asset::default()
    }
}

pub const STORAGE_BASE_KEY: u32 = openbrush::storage_unique_key!(BaseData);

/// The structure used to describe the Base
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_BASE_KEY)]
pub struct BaseData {
    /// List of all parts of Base.
    pub part_ids: Vec<PartId>,

    /// Mapping for all part details.
    pub parts: Mapping<PartId, Part>,

    /// Counter for assigning new parts to Base.
    pub next_part_id: PartId,

    /// Metadata for Base
    pub base_metadata_uri: String,
}

/// Part's details
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct Part {
    /// Part type `None`, `Slot` or `Fixed`.
    pub part_type: PartType,

    /// Depth used for composing parts
    pub z: u8,

    /// Collections that can be equipped into this part
    pub equippable: Vec<AccountId>,

    /// Uri for this part
    pub metadata_uri: String,

    /// Is accepting to be equipped by any collection
    pub is_equippable_by_all: bool,
}

/// Used to define a type of the part. Possible values are `None`, `Slot` or `Fixed`.
#[derive(
    scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, Clone, PartialEq,
)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub enum PartType {
    None,
    Slot,
    Fixed,
}
