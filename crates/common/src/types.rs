//! Types definition for RMRK contract

use ink_prelude::vec::Vec;
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};
use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        String,
    },
};

// Collection id is the address of child contract
pub type CollectionId = AccountId;

// Nft is a tuple of collection and TokenId and refers to the Child nft
pub type ChildNft = (CollectionId, Id);

pub type BaseId = u32;
pub type SlotId = u32;
pub type PartId = u32;
pub type AssetId = u32;
pub type EquippableGroupId = u32;

/// Part's details
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct Asset {
    /// Only used for assets meant to equip into others
    pub equippable_group_id: EquippableGroupId,

    /// metadata URI for Asset
    pub asset_uri: String,

    /// list of parts for this asset
    pub part_ids: Vec<PartId>,
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
    pub part_uri: String,

    /// Is accepting to be equipped by any collection
    pub is_equippable_by_all: bool,
}

/// Used to define a type of the part. Possible values are `None`, `Slot` or `Fixed`.
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub enum PartType {
    None,
    Slot,
    Fixed,
}

/// Used to define Equipment
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct Equipment {
    // asset_id: The ID of the asset being equipped by child
    pub asset_id: AssetId,

    // child_asset_id: The ID of the asset used as equipment
    pub child_asset_id: AssetId,

    // child_id: The (Address of the collection, token ID) of token that is equipped
    pub child_nft: ChildNft,
}
