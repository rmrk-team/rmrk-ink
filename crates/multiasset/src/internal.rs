use crate::{
    traits::MultiAssetEvents,
    MultiAssetData,
};

use rmrk_common::{
    errors::RmrkError,
    types::*,
    utils::Utils,
};

use ink_prelude::vec::Vec;

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        Storage,
        String,
    },
};

/// Trait definitions for MultiAsset helper functions
pub trait Internal {
    /// Check if asset is already accepted. Return error if it is
    fn ensure_not_accepted(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error>;

    /// Check if asset is already pending. Return error if it is
    fn ensure_not_pending(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error>;

    /// Check if asset is already pending. Return OK if it is
    fn ensure_pending(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error>;

    /// Check if asset is already accepted
    fn ensure_asset_accepted(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error>;

    /// Add the asset to the list of accepted assets
    fn add_to_accepted_assets(&mut self, token_id: &Id, asset_id: &AssetId);

    /// Add the asset to the list of pending assets
    fn add_to_pending_assets(&mut self, token_id: &Id, asset_id: &AssetId);

    /// Replace asset by another AssetId
    fn replace_asset(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
        replace_with_id: &AssetId,
    ) -> Result<(), PSP34Error>;

    /// Remove the asset to the list of pending assets
    fn remove_from_pending_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error>;

    /// Remove the asset to the list of accepted assets
    fn remove_from_accepted_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error>;
}

/// Implement internal helper trait for MultiAsset
impl<T> Internal for T
where
    T: Storage<MultiAssetData> + Storage<psp34::Data<enumerable::Balances>> + Utils,
{
    /// Check if asset is already accepted
    default fn ensure_not_accepted(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if children.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn ensure_not_pending(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn ensure_pending(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if !assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AssetIdNotFound.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already accepted
    default fn ensure_asset_accepted(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if !assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AssetIdNotFound.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Add the asset to the list of accepted assets
    default fn add_to_accepted_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(*asset_id);
            self.data::<MultiAssetData>()
                .accepted_assets
                .insert(&token_id, &assets);
        }
        self._emit_asset_accepted_event(token_id, asset_id);
    }

    /// Add the asset to the list of pending assets
    default fn add_to_pending_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .pending_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(*asset_id);
            self.data::<MultiAssetData>()
                .pending_assets
                .insert(&token_id, &assets);
        }
    }

    /// remove the asset from the list of pending assets
    default fn remove_from_pending_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut assets = self
            .data::<MultiAssetData>()
            .pending_assets
            .get(&token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        let index = assets
            .iter()
            .position(|a| a == asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidTokenId.as_str(),
            )))?;
        assets.remove(index);

        self.data::<MultiAssetData>()
            .pending_assets
            .insert(&token_id, &assets);

        Ok(())
    }

    /// Remove the asset from the list of accepted assets
    default fn remove_from_accepted_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        let index = assets
            .iter()
            .position(|a| a == asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidTokenId.as_str(),
            )))?;
        assets.remove(index);

        self.data::<MultiAssetData>()
            .accepted_assets
            .insert(&token_id, &assets);

        Ok(())
    }

    // TODO:
    // * add replace pending storage ( ie collection issuer might suggest asset replace on a token with different owner )
    // * add "upsert" operation ( if replace failed add as a new asset )
    default fn replace_asset(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
        replace_with_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut accepted_list = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::AcceptedAssetsMissing.as_str(),
            )))?;

        let asset_index = accepted_list
            .iter()
            .position(|x| x == replace_with_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        accepted_list[asset_index] = *asset_id;
        self.data::<MultiAssetData>()
            .accepted_assets
            .insert(&token_id, &accepted_list);

        Ok(())
    }
}
