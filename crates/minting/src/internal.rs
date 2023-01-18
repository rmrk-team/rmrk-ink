use crate::MintingData;

use rmrk_common::errors::RmrkError;

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        Storage,
        String,
    },
};

/// Trait definitions for Minting internal functions.
pub trait Internal {
    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;
}

/// Helper trait for Minting
impl<T> Internal for T
where
    T: Storage<MintingData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if the transferred mint values is as expected
    default fn _check_value(
        &self,
        transfered_value: u128,
        mint_amount: u64,
    ) -> Result<(), PSP34Error> {
        if let Some(value) =
            (mint_amount as u128).checked_mul(self.data::<MintingData>().price_per_mint)
        {
            if transfered_value == value {
                return Ok(())
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::BadMintValue.as_str(),
        )))
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::CannotMintZeroTokens.as_str(),
            )))
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
        return Err(PSP34Error::Custom(String::from(
            RmrkError::CollectionIsFull.as_str(),
        )))
    }
}
