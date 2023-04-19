use crate::Catalog;

use ink::prelude::{
    vec,
    vec::Vec,
};

use openbrush::{
    contracts::access_control::{
        self,
        only_role,
    },
    modifiers,
    traits::Storage,
};

use rmrk_common::{
    counter::Counter,
    errors::{
        Result,
        RmrkError,
    },
    roles::CONTRIBUTOR,
    types::*,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(CatalogAutoIndex);

/// Storage for AutoIndex `PartId` counter
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct CatalogAutoIndexData {
    pub part_id: Counter<PartId>,
}

#[openbrush::wrapper]
pub type CatalogAutoIndexRef = dyn CatalogAutoIndex;

#[openbrush::trait_definition]
pub trait CatalogAutoIndex {
    /// Add one or more parts to the Catalog, with auto-generated PartIds
    #[ink(message)]
    fn add_part_list(&mut self, parts: Vec<Part>) -> Result<(PartId, PartId)>;
}

impl<T> CatalogAutoIndex for T
where
    T: Storage<access_control::Data> + Storage<CatalogAutoIndexData> + Catalog,
{
    /// Add one or more parts to the Catalog, with auto-generated PartIds
    /// The returned range provides the first and last generated PartId
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn add_part_list(&mut self, parts: Vec<Part>) -> Result<(PartId, PartId)> {
        let mut part_ids = vec![];

        for _ in 0..parts.len() {
            let next_id = self.data::<CatalogAutoIndexData>().part_id.next()?;
            part_ids.push(next_id);
        }

        let range_start = *part_ids.first().ok_or(RmrkError::BadConfig)?;
        let range_end = *part_ids.last().ok_or(RmrkError::BadConfig)?;

        Catalog::add_part_list(self, part_ids, parts)?;

        Ok((range_start, range_end))
    }
}
