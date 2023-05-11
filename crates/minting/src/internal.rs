use crate::MintingData;

use rmrk_common::errors::{
    Result,
    RmrkError,
};

use ink::prelude::string::String as PreludeString;

use openbrush::{
    contracts::psp34::{
        balances::BalancesManager,
        extensions::enumerable::*,
    },
    traits::Storage,
};

/// Trait definitions for Minting internal functions.
pub trait Internal {
    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<()>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<()>;

    /// Get URI for the token Id.
    fn _token_uri(&self, token_id: u64) -> Result<PreludeString>;
}

/// Helper trait for Minting
impl<T> Internal for T
where
    T: Storage<MintingData>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal
        + Storage<psp34::Data<enumerable::Balances>>,
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

        Err(RmrkError::BadMintValue.into())
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<()> {
        if mint_amount == 0 {
            return Err(RmrkError::CannotMintZeroTokens.into())
        }

        if let Some(amount) = self
            .data::<psp34::Data<enumerable::Balances>>()
            .balances
            .total_supply()
            .checked_add(mint_amount as u128)
        {
            return match self.data::<MintingData>().max_supply {
                Some(max_supply) if amount <= max_supply as u128 => Ok(()),
                Some(0) | None => Ok(()),
                _ => Err(RmrkError::CollectionIsFull.into()),
            }
        }

        Err(RmrkError::CollectionIsFull.into())
    }

    /// Get URI for the token Id.
    default fn _token_uri(&self, token_id: u64) -> Result<PreludeString> {
        self.data::<MintingData>()
            .nft_metadata
            .get(Id::U64(token_id))
            .and_then(|token_uri| PreludeString::from_utf8(token_uri).ok())
            .ok_or(RmrkError::UriNotFound.into())
    }
}
