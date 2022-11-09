use ink_prelude::string::String;
use openbrush::traits::Balance;
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    CannotMintZeroTokens,
    CollectionFullOrLocked,
    MintUnderpriced,
}

impl RmrkError {
    pub fn as_str(&self) -> String {
        match self {
            RmrkError::CannotMintZeroTokens => "CannotMintZeroTokens".to_string(),
            RmrkError::CollectionFullOrLocked => "CollectionFullOrLocked".to_string(),
            RmrkError::MintUnderpriced => "MintUnderpriced".to_string(),
        }
    }
}
