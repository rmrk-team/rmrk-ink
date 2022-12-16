//! RMRK Base traits

use crate::impls::rmrk::types::*;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::PSP34Error,
    traits::{
        AccountId,
        String,
    },
};

/// Trait definitions for Base functions
#[openbrush::trait_definition]
pub trait Base {
    /// Add one or more parts to the base
    #[ink(message)]
    fn add_part_list(&mut self, part: Vec<Part>) -> Result<(), PSP34Error>;

    /// Mint one or more tokens.
    #[ink(message)]
    fn add_equipable_addresses(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error>;

    /// Mint one or more tokens.
    #[ink(message)]
    fn set_equipable_addresses(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error>;

    /// Remove list of equipable addresses for given Part
    #[ink(message)]
    fn reset_equipable_address(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error>;

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[ink(message)]
    fn set_equippable_by_all(&self, part_id: PartId) -> Result<(), PSP34Error>;

    //// Get the Base metadataURI.
    #[ink(message)]
    fn get_base_metadata(&self) -> String;

    /// Get the part type for the given PartId. It can be None, Fixed, Slot.
    #[ink(message)]
    fn get_part_type(&self, part_id: PartId) -> PartType;

    /// Get the list of all parts.
    #[ink(message)]
    fn get_all_parts(&self) -> Vec<PartId>;

    /// Get the part details for the given PartId.
    #[ink(message)]
    fn get_part(&self, part_id: PartId) -> Part;

    /// Check whether the given address is allowed to equip the desired `PartId`.
    #[ink(message)]
    fn is_equippable(&self, part_id: PartId, target_address: AccountId) -> bool;

    /// Checks if is_equippable_by_all is set to true for the given `PartId`
    #[ink(message)]
    fn is_equippable_by_all(&self, part_id: PartId) -> bool;
}
