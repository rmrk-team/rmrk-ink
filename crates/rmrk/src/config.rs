use openbrush::{
    contracts::{
        access_control::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        StorageAsMut,
        StorageAsRef,
        String,
    },
};

use rmrk_common::roles::CONTRIBUTOR;
use rmrk_minting::{
    self,
    traits::MintingLazy,
};

pub fn with_collection<T>(
    instance: &mut T,
    name: String,
    symbol: String,
    base_uri: String,
    metadata: String,
    max_supply: u64,
) where
    T: Storage<rmrk_minting::MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<metadata::Data>,
{
    let minting: &mut rmrk_minting::MintingData = <T as StorageAsMut>::data(instance);
    minting.max_supply = max_supply;
    let psp34: &psp34::Data<enumerable::Balances> = <T as StorageAsRef>::data(instance);
    let collection_id = psp34.collection_id();
    instance._set_attribute(collection_id.clone(), String::from("name"), name);
    instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
    instance._set_attribute(collection_id.clone(), String::from("baseUri"), base_uri);
    instance._set_attribute(collection_id, String::from("collection_metadata"), metadata);
}

pub fn with_lazy_mint<T>(instance: &mut T, price_per_mint: Balance)
where
    T: MintingLazy
        + Storage<rmrk_minting::MintingData>
        + Storage<psp34::Data<enumerable::Balances>>,
{
    let minting: &mut rmrk_minting::MintingData = <T as StorageAsMut>::data(instance);

    minting.price_per_mint = price_per_mint;
}

pub fn with_admin<T>(instance: &mut T, account: AccountId)
where
    T: access_control::Internal + Storage<access_control::Data>,
{
    instance._init_with_admin(account);
    instance._setup_role(CONTRIBUTOR, account);
}

pub fn with_contributor<T>(instance: &mut T, account: AccountId)
where
    T: access_control::Internal + Storage<access_control::Data>,
{
    instance._setup_role(CONTRIBUTOR, account);
}

// pub fn with_parts<T>(instance: &mut T, parts: Vec<Part>) -> Result<(), PSP34Error>

// {
//     BaseRef::add_part_list(parts)?;
//     Ok(())
// }

fn _with_royalties<T>(_instance: &mut T, _royalty_receiver: AccountId, _royalty: u8) {
    todo!()
}
