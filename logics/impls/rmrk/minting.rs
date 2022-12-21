//! RMRK minting implementation

use crate::impls::rmrk::{
    errors::RmrkError,
    types::MintingData,
};
pub use crate::traits::minting::{
    Internal,
    Minting,
};
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

impl<T> Minting for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    /// Mint next available token for the caller
    default fn mint_next(&mut self) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), 1)?;
        let caller = Self::env().caller();
        let token_id = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(1)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::CollectionIsFull.as_str(),
            )))?;
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(caller, Id::U64(token_id))?;
        self.data::<MintingData>().last_token_id += 1;

        self._emit_transfer_event(None, Some(caller), Id::U64(token_id));
        return Ok(())
    }

    /// Mint one or more tokens
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        self._check_amount(mint_amount)?;

        let next_to_mint = self.data::<MintingData>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            self.data::<psp34::Data<enumerable::Balances>>()
                ._mint_to(to, Id::U64(mint_id))?;
            self.data::<MintingData>().last_token_id += 1;
            self._emit_transfer_event(None, Some(to), Id::U64(mint_id));
        }

        Ok(())
    }
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
