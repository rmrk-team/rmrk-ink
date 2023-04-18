use crate::Catalog;

use ink::prelude::vec::Vec;

use openbrush::traits::Storage;

use rmrk_common::{
    counter::Counter,
    errors::{
        Result,
        RmrkError,
    },
    types::*,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(CatalogAutoIndex);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct CatalogAutoIndexData {
    pub part_id: Counter<PartId>,
}

#[openbrush::wrapper]
pub type CatalogAutoIndexRef = dyn CatalogAutoIndex;

#[openbrush::trait_definition]
pub trait CatalogAutoIndex {
    #[ink(message)]
    fn add_part_list(&mut self, parts: Vec<Part>) -> Result<(PartId, PartId)>;
}

impl<T> CatalogAutoIndex for T
where
    T: Storage<CatalogAutoIndexData> + Catalog,
{
    default fn add_part_list(&mut self, parts: Vec<Part>) -> Result<(PartId, PartId)> {
        let mut part_ids = vec![];

        for _ in 0..parts.len() {
            let next_id = self.data::<CatalogAutoIndexData>().part_id.next()?;
            part_ids.push(next_id);
        }

        let range_start = part_ids.first().ok_or(RmrkError::BadConfig)?.clone();
        let range_end = part_ids.last().ok_or(RmrkError::BadConfig)?.clone();

        Catalog::add_part_list(self, part_ids, parts)?;

        Ok((range_start, range_end))
    }
}
