use crate::MintingData;

use rmrk_common::errors::{
    Result,
    RmrkError,
};

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        Storage,
    },
};

/// Trait definitions for Minting internal functions.
pub trait Internal {
    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<()>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<()>;
}

/// Helper trait for Minting
impl<T> Internal for T
where
    T: Storage<MintingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if the transferred mint values is as expected
    default fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<()> {
        if let Some(value) =
            (mint_amount as u128).checked_mul(self.data::<MintingData>().price_per_mint)
        {
            if transfered_value == value {
                return Ok(())
            }
        }
        return Err(RmrkError::BadMintValue.into())
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<()> {
        if mint_amount == 0 {
            return Err(RmrkError::CannotMintZeroTokens.into())
        }
        if let Some(amount) = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(mint_amount)
        {
            if amount <= self.data::<MintingData>().max_supply {
                return Ok(())
            }
        }
        return Err(RmrkError::CollectionIsFull.into())
    }
}
