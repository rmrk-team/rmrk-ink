//! RMRK Equippable implementation

use crate::impls::rmrk::{
    errors::RmrkError,
    types::*,
};
pub use crate::traits::{
    base::Base,
    equippable::{
        Equippable,
        EquippableEvents,
        Internal,
    },
    multiasset::{
        Internal as MultiAssetInternal,
        MultiAsset,
    },
    utils::Utils,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::enumerable::*,
    },
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Implement internal helper trait for Equippable
impl<T> Internal for T
where
    T: Storage<EquippableData>
        + Storage<ownable::Data>
        + Storage<MultiAssetData>
        + MultiAsset
        + MultiAssetInternal
        + Storage<BaseData>
        + Utils,
{
    /// Check if slot is already used/equipped.
    default fn ensure_token_slot_free(
        &self,
        token_id: &Id,
        part_id: &PartId,
    ) -> Result<(), PSP34Error> {
        if (self
            .data::<EquippableData>()
            .equipment
            .get((token_id, part_id)))
        .is_some()
        {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::SlotAlreayUsed.as_str(),
            )))
        }
        Ok(())
    }
    /// Check if asset is already added.
    default fn ensure_asset_accepts_slot(
        &self,
        asset_id: &AssetId,
        part_id: &PartId,
    ) -> Result<(), PSP34Error> {
        if let Some(equippable_asset) = self.data::<EquippableData>().equippable_asset.get(asset_id)
        {
            if !equippable_asset.port_ids.contains(part_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::TargetAssetCannotReceiveSlot.as_str(),
                )))
            }
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::AssetHasNoParts.as_str(),
            )))
        }
        Ok(())
    }

    // Used to ensure a token can be equipped into a given parent's slot. Check in child NFT
    fn ensure_token_can_be_equipped_with_asset_into_slot(
        &self,
        parent_address: AccountId,
        token_id: Id,
        asset_id: AssetId,
        _part_slot_id: PartId,
    ) -> Result<(), PSP34Error> {
        if let Some(equippable_asset) = self.data::<EquippableData>().equippable_asset.get(asset_id)
        {
            let equippable_group_id = equippable_asset.group_id;
            if self
                .data::<EquippableData>()
                .valid_parent_slot
                .get((equippable_group_id, parent_address))
                .is_some()
            {
                self.ensure_asset_accepted(&token_id, &asset_id)?;
            } else {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::UnknownPart.as_str(),
                )))
            }
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::UnknownEquippableAsset.as_str(),
            )))
        }
        Ok(())
    }

    /// Used to ensure a token is not equipped and can be un-equipped.
    fn ensure_equipped(
        &self,
        token_id: &Id,
        slot_part_id: &PartId,
    ) -> Result<Equipment, PSP34Error> {
        if let Some(equipment) = self
            .data::<EquippableData>()
            .equipment
            .get((token_id, slot_part_id))
        {
            return Ok(equipment)
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::NotEquipped.as_str(),
            )))
        }
    }
}

impl<T> Equippable for T
where
    T: Storage<EquippableData>
        + Storage<ownable::Data>
        + Storage<MultiAssetData>
        + MultiAsset
        + MultiAssetInternal
        + Storage<BaseData>
        + Utils,
{
    /// Used to equip a child nft into a token.
    default fn equip(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        slot_part_id: PartId,
        child_nft: ChildNft,
        child_asset_id: AssetId,
    ) -> Result<(), PSP34Error> {
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        self.ensure_asset_accepts_slot(&asset_id, &slot_part_id)?;
        self.ensure_token_slot_free(&token_id, &slot_part_id)?;

        // TODO Cross contract call to check from child prespective. At the same time this will not work for PSP34 child
        // EquippableRef::ensure_token_can_be_equipped_with_asset_into_slot(child_nft.0, Self::env().account_id(),
        //     child_nft.1,
        //     child_asset_id,
        //     slot_part_id)?;

        // Check from base perspective
        self.ensure_equippable(slot_part_id, child_nft.0)?;

        // insert equipment
        let equipment = Equipment {
            asset_id,
            child_asset_id,
            child_nft: child_nft.clone(),
        };
        self.data::<EquippableData>()
            .equipment
            .insert((token_id.clone(), slot_part_id), &equipment);

        self.emit_child_asset_equipped(token_id, asset_id, slot_part_id, child_nft, child_asset_id);
        Ok(())
    }

    /// Used to unequip child from parent token.
    default fn unequip(&mut self, token_id: Id, slot_part_id: PartId) -> Result<(), PSP34Error> {
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        let equipment = self.ensure_equipped(&token_id, &slot_part_id)?;
        let asset_id = equipment.asset_id;

        self.data::<EquippableData>()
            .equipment
            .remove((token_id.clone(), slot_part_id));

        self.emit_child_asset_unequipped(token_id, asset_id, slot_part_id);
        Ok(())
    }

    /// Used to declare that the assets belonging to a given `equippableGroupId` are equippable into the `Slot`
    /// associated with the `partId` of the collection at the specified `parentAddress`
    default fn set_valid_parent_for_equippable_group(
        &mut self,
        equippable_group_id: EquippableGroupId,
        parent_address: AccountId,
        part_id: PartId,
    ) -> Result<(), PSP34Error> {
        self.data::<EquippableData>()
            .valid_parent_slot
            .insert((equippable_group_id, parent_address), &part_id);
        self.emit_valid_parent_equippable_group_set(equippable_group_id, part_id, parent_address);

        Ok(())
    }

    /// Used to extend already added Asset with details needed to support equipping.
    default fn extend_equippable_asset(
        &mut self,
        asset_id: AssetId,
        group_id: EquippableGroupId,
        port_ids: Vec<PartId>,
    ) -> Result<(), PSP34Error> {
        if self.asset_id_exists(asset_id).is_none() {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::AssetIdNotFound.as_str(),
            )))
        }
        self.data::<EquippableData>()
            .equippable_asset
            .insert(&asset_id, &EquippableAsset { group_id, port_ids });
        Ok(())
    }

    /// Used to get the Equipment object equipped into the specified slot of the desired token.
    default fn get_equipment(
        &self,
        token_id: Id,
        slot_part_id: PartId,
    ) -> Result<Equipment, PSP34Error> {
        self.data::<EquippableData>()
            .equipment
            .get((token_id, slot_part_id))
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::EquipmentNotFound.as_str(),
            )))
    }

    /// Used to get the asset and equippable data associated with given `asset_id`.
    default fn get_asset_and_equippable_data(
        &self,
        token_id: Id,
        asset_id: AssetId,
    ) -> Result<(Option<String>, EquippableAsset), PSP34Error> {
        self.ensure_asset_accepted(&token_id, &asset_id)?;
        if let Some(e) = self.data::<EquippableData>().equippable_asset.get(asset_id) {
            let uri = self.get_asset_uri(asset_id);
            return Ok((uri, e))
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::AssetIdNotFound.as_str(),
            )))
        }
    }
}

impl<T> EquippableEvents for T {
    /// Used to notify listeners that a child's asset has been equipped into one of its parent assets.
    default fn emit_child_asset_equipped(
        &self,
        _token_id: Id,
        _asset_id: AssetId,
        _slot_part_id: PartId,
        _child_nft: ChildNft,
        _child_asset_id: AssetId,
    ) {
    }

    /// Used to notify listeners that an asset object at `asset_id` is added to token's pending asset
    default fn emit_child_asset_unequipped(
        &self,
        _token_id: Id,
        _asset_id: AssetId,
        _slot_part_id: PartId,
    ) {
    }

    /// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
    /// equippable into a given slot and parent
    default fn emit_valid_parent_equippable_group_set(
        &self,
        _group_id: EquippableGroupId,
        _slot_part_id: PartId,
        _parent_address: AccountId,
    ) {
    }
}
