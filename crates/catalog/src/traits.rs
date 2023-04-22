//! RMRK Catalog traits

use rmrk_common::{
    errors::Result,
    types::*,
};

use ink::prelude::{
    string::String as PreludeString,
    vec::Vec,
};
use openbrush::traits::{
    AccountId,
    String,
};

#[openbrush::wrapper]
pub type CatalogRef = dyn Catalog;

/// Trait definitions for Catalog
#[openbrush::trait_definition]
pub trait Catalog {
    /// Add one or more parts to the Catalog
    #[ink(message)]
    fn add_part_list(&mut self, parts: Vec<Part>) -> Result<()>;

    /// Add collection address(es) that can be used to equip given `PartId`.
    #[ink(message)]
    fn add_equippable_addresses(
        &mut self,
        part_id: PartId,
        equippable_address: Vec<AccountId>,
    ) -> Result<()>;

    /// Remove list of equippable addresses for given Part
    #[ink(message)]
    fn reset_equippable_addresses(&mut self, part_id: PartId) -> Result<()>;

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[ink(message)]
    fn set_equippable_by_all(&mut self, part_id: PartId) -> Result<()>;

    //// Set the Catalog metadataURI.
    #[ink(message)]
    fn set_catalog_metadata(&mut self, catalog_metadata: String) -> Result<()>;

    //// Get the Catalog metadataURI.
    #[ink(message)]
    fn get_catalog_metadata(&self) -> Result<PreludeString>;

    /// Get the list of all parts.
    #[ink(message)]
    fn get_parts_count(&self) -> PartId;

    /// Get the part details for the given PartId.
    #[ink(message)]
    fn get_part(&self, part_id: PartId) -> Option<Part>;

    /// Check whether the given address is allowed to equip the desired `PartId`.
    #[ink(message)]
    fn ensure_equippable(&self, part_id: PartId, target_address: AccountId) -> Result<()>;

    /// Checks if the given `PartId` can be equipped by any collection
    #[ink(message)]
    fn is_equippable_by_all(&self, part_id: PartId) -> bool;
}
