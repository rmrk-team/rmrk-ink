//! Trait definitions for Equippable module
use openbrush::{
    contracts::psp34::Id,
    traits::AccountId,
};
use rmrk_common::{
    errors::Result,
    types::*,
};

#[openbrush::wrapper]
pub type EquippableRef = dyn Equippable;

/// Trait definitions for Equipping RMRK NFTs
#[openbrush::trait_definition]
pub trait Equippable {
    /// Used to equip a child nft into a token.
    /// # Requirements
    ///  * Called on Parent token contract
    ///  * If the `Slot` already has an item equipped, the execution will be reverted.
    ///  * If the child can't be used in the given `Slot`, the execution will be reverted.
    ///  * If the base doesn't allow this equip to happen, the execution will be reverted.
    ///    
    /// # Arguments:
    ///  * `token_id ID` of the token that had an asset equipped
    ///  * `asset_id ID` of the asset associated with the token we are equipping into
    ///  * `slot_part_id` ID of the slot we are using to equip
    ///  * `child_nft` Child NFT tuple (CollectionId, Id)
    ///  * `child_asset_id` ID of the asset associated with the token we are equipping
    /// Emits an {ChildAssetEquipped} event.
    #[ink(message)]
    fn equip(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        slot_part_id: PartId,
        child_nft: ChildNft,
        child_asset_id: AssetId,
    ) -> Result<()>;

    /// Used to unequip child from parent token.
    /// # Requirements
    ///  * This can only be called by the owner of the token or by an account that has been granted permission to
    ///  * Called on Parent token contract
    ///
    /// # Arguments:
    ///  * `token_id` ID of the token that had an asset unequipped
    ///  * `asset_id` ID of the asset associated with the token we are unequipping from
    ///  * `slot_part_id` ID of the slot we are using to unequip
    ///  * `child_nft` Child NFT tuple (CollectionId, Id)
    ///  * `child_asset_id` ID of the asset associated with the token we are unequipping
    /// Emits an {ChildAssetUnequipped} event.
    #[ink(message)]
    fn unequip(&mut self, token_id: Id, slot_part_id: PartId) -> Result<()>;

    /// Used to declare that the assets belonging to a given `equippableGroupId` are equippable into the `Slot`
    /// associated with the `partId` of the collection at the specified `parentAddress`
    /// # Requirements
    ///  * Called on Child Token contract
    ///
    /// # Arguments:
    ///  * `equippable_group_id` ID of the equippable group
    ///  * `parent_address` Address of the parent into which the equippable group can be equipped into
    ///  * `part_id` ID of the `Slot` that the items belonging to the equippable group can be equipped into
    #[ink(message)]
    fn set_valid_parent_for_equippable_group(
        &mut self,
        equippable_group_id: EquippableGroupId,
        parent_address: AccountId,
        part_id: PartId,
    ) -> Result<()>;

    /// Used to extend already added Asset with details needed to support equipping.
    /// These details are not present in MultiAsset trait to avoid dependencies on Equippable trait.
    /// # Arguments:
    ///  * `asset_id` ID of the asset being extended
    ///  * `equippableGroupId` ID of the equippable group
    ///  * `partIds` An array of IDs of fixed and slot parts to be included in the asset
    // #[ink(message)]
    // fn extend_equippable_asset(
    //     &mut self,
    //     asset_id: AssetId,
    //     group_id: EquippableGroupId,
    //     port_ids: Vec<PartId>,
    // ) -> Result<()>;

    /// Used to get the Equipment object equipped into the specified slot of the desired token.
    ///
    /// # Arguments:
    ///  * `token_id` ID of the token for which we are retrieving the equipped object
    ///  * `slot_part_id` ID of the `Slot` part that we are checking for equipped objects
    #[ink(message)]
    fn get_equipment(&self, token_id: Id, slot_part_id: PartId) -> Option<Equipment>;

    /// Used to get the asset and equippable data associated with given `asset_id`.
    /// # Arguments:
    ///  * tokenId ID of the token for which to retrieve the asset
    ///  * asset_id ID of the asset of which we are retrieving
    /// # Returns:
    ///    * asset_id metadataURI,
    ///    * EquippableAsset
    ///    * catalog address
    #[ink(message)]
    fn get_asset_and_equippable_data(&self, token_id: Id, asset_id: AssetId) -> Result<Asset>;

    /// Used to ensure a token can be equipped into a given parent's slot.
    /// # Arguments:
    ///  * parent Address of the parent token's smart contract
    ///  * tokenId ID of the token we want to equip
    ///  * asset_id ID of the asset associated with the token we want to equip
    ///  * slotId ID of the slot that we want to equip the token into
    /// * @return bool The boolean indicating whether the token with the given asset can be equipped into the desired
    #[ink(message)]
    fn ensure_token_can_be_equipped_with_asset_into_slot(
        &self,
        parent_address: AccountId,
        parent_token_id: Id,
        asset_id: AssetId,
        slot_part_id: PartId,
    ) -> Result<()>;
}

/// Trait definitions for Resource ink events
#[openbrush::trait_definition]
pub trait EquippableEvents {
    /// Used to notify listeners that a child's asset has been equipped into one of its parent assets.
    /// # Arguments:
    ///  * token_id ID of the token that had an asset equipped
    ///  * asset_id ID of the asset associated with the token we are equipping into
    ///  * slot_part_id ID of the slot we are using to equip
    ///  * child_nft Child NFT tuple (CollectionId, Id)
    ///  * child_asset_id ID of the asset associated with the token we are equipping
    fn emit_child_asset_equipped(
        &self,
        token_id: Id,
        asset_id: AssetId,
        slot_part_id: PartId,
        child_nft: ChildNft,
        child_asset_id: AssetId,
    );

    /// Used to notify listeners that an asset object at `asset_id` is added to token's pending asset
    /// # Arguments:
    ///  * token_id ID of the token that had an asset unequipped
    ///  * asset_id ID of the asset associated with the token we are unequipping from
    ///  * slot_part_id ID of the slot we are using to unequip
    ///  * child_nft Child NFT tuple (CollectionId, Id)
    ///  * child_asset_id ID of the asset associated with the token we are unequipping
    fn emit_child_asset_unequipped(&self, token_id: Id, asset_id: AssetId, slot_part_id: PartId);

    //// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
    /// equippable into a given slot and parent
    /// # Arguments:
    ///  * `equippableGroupId` ID of the equippable group being marked as equippable into the slot associated with
    ///    `slotPartId` of the `parentAddress` collection
    ///  * `slotPartId` ID of the slot part of the base into which the parts belonging to the equippable group
    ///     associated with `equippableGroupId` can be equipped
    ///  * `parentAddress` Address of the collection into which the parts belonging to `equippableGroupId` can be
    ///     equipped
    fn emit_valid_parent_equippable_group_set(
        &self,
        group_id: EquippableGroupId,
        slot_part_id: PartId,
        parent_address: AccountId,
    );
}
