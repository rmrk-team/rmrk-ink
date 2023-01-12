//! Trait definitions for MultiAsset module
use crate::impls::rmrk::types::*;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::{
        Id,
        PSP34Error,
    },
    traits::String,
};

#[openbrush::wrapper]
pub type MultiAssetRef = dyn MultiAsset;

/// Trait definitions for MultiAsset ink! messages
#[openbrush::trait_definition]
pub trait MultiAsset {
    /// Used to add a asset entry.
    /// The ID of the asset is automatically assigned to be the next available asset ID.
    /// # Arguments
    ///  * `asset_uri` Uri for the new asset
    /// Emits an {AssetSet} event.
    #[ink(message)]
    fn add_asset_entry(
        &mut self,
        id: AssetId,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<(), PSP34Error>;

    /// Used to add an asset to a token.
    /// If the given asset is already added to the token, the execution will be reverted.
    /// If the asset ID is invalid, the execution will be reverted.
    /// If the token already has the maximum amount of pending assets (128), the execution will be
    /// reverted.
    /// If the asset is being added by the current root owner of the token, the asset will be automatically
    /// accepted.
    /// # Arguments
    ///  * tokenId ID of the token to add the asset to
    ///  * assetId ID of the asset to add to the token
    ///  * replacesAssetWithId ID of the asset to replace from the token's list of active assets
    /// Emits an {AssetAddedToToken} event.

    #[ink(message)]
    fn add_asset_to_token(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        replaces_asset_with_id: Option<Id>,
    ) -> Result<(), PSP34Error>;

    /// Accepts an asset at from the pending array of given token.
    /// Migrates the asset from the token's pending asset array to the token's active asset array.
    /// Active assets cannot be removed by anyone, but can be replaced by a new asset.
    /// # Requirements:
    ///  * The caller must own the token or be approved to manage the token's assets
    ///  * `tokenId` must exist.
    ///  * `assetId` must be in the pending_asset list.
    /// # Arguments
    ///  * tokenId ID of the token for which to accept the pending asset
    ///  * assetId ID of the asset expected to be in the pending_asset list.
    /// Emits an {AssetAccepted} event.
    #[ink(message)]
    fn accept_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error>;

    /// Rejects an asset from the pending array of given token.
    /// Removes the asset from the token's pending asset array.
    /// # Requirements:
    ///  * The caller must own the token or be approved to manage the token's assets
    ///  * `tokenId` must exist.
    ///  * `assetId` must be in the pending_asset list.
    /// # Arguments
    ///  * tokenId ID of the token for which to accept the pending asset
    ///  * assetId ID of the asset expected to be in the pending_asset list.
    /// Emits a {AssetRejected} event.
    #[ink(message)]
    fn reject_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error>;

    /// Used to specify the priorities for a given token's active assets.
    /// If the length of the priorities array doesn't match the length of the active assets array, the execution
    ///  will be reverted.
    /// The position of the priority value in the array corresponds the position of the asset in the active
    ///  assets array it will be applied to.
    /// # Arguments
    ///  * tokenId ID of the token for which the priorities are being set
    ///  * priorities Array of priorities for the assets
    /// Emits a {AssetPrioritySet} event.
    #[ink(message)]
    fn set_priority(&mut self, token_id: Id, priorities: Vec<AssetId>) -> Result<(), PSP34Error>;

    /// Used to retrieve the total number of assets.
    /// # Returns
    ///  * u64 The total number of assets
    #[ink(message)]
    fn total_assets(&self) -> u32;

    /// Used to retrieve asset's uri
    #[ink(message)]
    fn get_asset_uri(&self, asset_id: AssetId) -> Option<String>;

    /// Used to retrieve the total number of assets per token
    #[ink(message)]
    fn total_token_assets(&self, token_id: Id) -> Result<(u64, u64), PSP34Error>;

    /// Fetch all accepted assets for the token_id
    #[ink(message)]
    fn get_accepted_token_assets(&self, token_id: Id) -> Result<Option<Vec<AssetId>>, PSP34Error>;

    /// Remove the assets for the list of token assets
    #[ink(message)]
    fn remove_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error>;

    /// Check that asset id does not already exist.
    fn ensure_asset_id_is_available(&self, asset_id: AssetId) -> Result<(), PSP34Error>;
}

/// Trait definitions for MultiAsset helper functions
#[openbrush::trait_definition]
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

/// Trait definitions for MultiAsset ink events
#[openbrush::trait_definition]
pub trait MultiAssetEvents {
    /// Used to notify listeners that an asset object is initialized at `assetId`.
    /// # Arguments:
    /// * assetId ID of the asset that was initialized
    fn _emit_asset_set_event(&self, asset_id: &AssetId);

    /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset
    /// array.
    /// # Arguments:
    /// * tokenId ID of the token that received a new pending asset
    /// * assetId ID of the asset that has been added to the token's pending assets array
    /// * replacesId ID of the asset that would be replaced
    fn _emit_asset_added_to_token_event(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
        replaces_id: Option<Id>,
    );

    /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
    /// from token's pending assets array to active assets array of the token.
    /// # Arguments:
    /// * tokenId ID of the token that had a new asset accepted
    /// * assetId ID of the asset that was accepted
    /// * replacesId ID of the asset that was replaced
    fn _emit_asset_accepted_event(&self, token_id: &Id, asset_id: &AssetId);

    /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped
    /// from the pending assets array of the token.
    /// # Arguments
    /// * tokenId ID of the token that had an asset rejected
    /// * assetId ID of the asset that was rejected
    fn _emit_asset_rejected_event(&self, token_id: &Id, asset_id: &AssetId);

    /// Used to notify listeners that an asset object at `assetId` is removed from token
    /// # Arguments
    /// * tokenId ID of the token that had an asset rejected
    /// * assetId ID of the asset that was rejected
    fn _emit_asset_removed_event(&self, token_id: &Id, asset_id: &AssetId);

    /// Used to notify listeners that token's prioritiy array is reordered.
    /// # Arguments
    /// * tokenId ID of the token that had the asset priority array updated
    fn _emit_asset_priority_set_event(&self, token_id: &Id, priorities: Vec<AssetId>);
}
