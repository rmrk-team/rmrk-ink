//! RMRK Base traits

use crate::impls::rmrk::types::*;
use ink_prelude::{
    string::{
        String as PreludeString,
    },
    vec::Vec,
};
use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{
        AccountId,
        String,
    },
};

/// Implement internal helper trait for MultiAsset
pub trait Internal {
    fn ensure_only_slot(&self, part_id: PartId) -> Result<(), PSP34Error>;
}
/// Trait definitions for Base functions
#[openbrush::trait_definition]
pub trait Base {
    /// Add one or more parts to the base
    #[ink(message)]
    fn add_part_list(&mut self, parts: Vec<Part>) -> Result<(), PSP34Error>;

    /// Add collection address(es) that can be used to equip given `PartId`.
    #[ink(message)]
    fn add_equipable_addresses(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error>;

    /// Remove list of equipable addresses for given Part
    #[ink(message)]
    fn reset_equipable_addresses(&mut self, part_id: PartId) -> Result<(), PSP34Error>;

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[ink(message)]
    fn set_equippable_by_all(&mut self, part_id: PartId) -> Result<(), PSP34Error>;

    //// Set the Base metadataURI.
    #[ink(message)]
    fn set_base_metadata(&mut self, base_metadata: String) -> Result<(), PSP34Error>;

    //// Get the Base metadataURI.
    #[ink(message)]
    fn get_base_metadata(&self) -> PreludeString;

    /// Get the list of all parts.
    #[ink(message)]
    fn get_parts_count(&self) -> PartId;

    /// Get the part details for the given PartId.
    #[ink(message)]
    fn get_part(&self, part_id: PartId) -> Option<Part>;

    /// Check whether the given address is allowed to equip the desired `PartId`.
    #[ink(message)]
    fn is_equippable(&self, part_id: PartId, target_address: AccountId) -> bool;

    /// Checks if is_equippable_by_all is set to true for the given `PartId`
    #[ink(message)]
    fn is_equippable_by_all(&self, part_id: PartId) -> bool;
}
