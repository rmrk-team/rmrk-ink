use openbrush::traits::Balance;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub max_supply: u128,
    pub price_per_mint: Balance,
}
