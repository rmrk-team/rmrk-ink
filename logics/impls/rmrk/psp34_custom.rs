//! Set of functions commonly used with PSP34 contract
//!

// Copyright (c) 2022 Astar Network
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use ink_prelude::string::{String as PreludeString, ToString};

use crate::impls::rmrk::psp34_custom_types::{Data, RmrkError};
pub use crate::traits::psp34_custom::Psp34Custom;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{enumerable::*, metadata::*},
        reentrancy_guard::*,
    },
    modifiers,
    traits::{AccountId, Balance, Storage, String},
};

pub trait Internal {
    /// Check if the transferred mint values is as expected
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check if token is minted
    fn _token_exists(&self, id: Id) -> Result<(), PSP34Error>;

    // /// Emit Transfer event
    // fn _emit_tra?event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool);
}

impl<T> Psp34Custom for T
where
    T: Storage<Data>
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
        let token_id =
            self.data::<Data>()
                .last_token_id
                .checked_add(1)
                .ok_or(PSP34Error::Custom(String::from(
                    RmrkError::CollectionIsFull.as_str(),
                )))?;
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(caller, Id::U64(token_id))?;
        self.data::<Data>().last_token_id += 1;

        self._emit_transfer_event(None, Some(caller), Id::U64(token_id));
        return Ok(());
    }

    /// Mint one or more tokens
    #[modifiers(non_reentrant)]
    default fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        self._check_amount(mint_amount)?;

        let next_to_mint = self.data::<Data>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            assert!(self
                .data::<psp34::Data<enumerable::Balances>>()
                ._mint_to(to, Id::U64(mint_id))
                .is_ok());
            self.data::<Data>().last_token_id += 1;
            self._emit_transfer_event(None, Some(to), Id::U64(mint_id));
        }

        Ok(())
    }

    /// Set new value for the baseUri
    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        let id = self
            .data::<psp34::Data<enumerable::Balances>>()
            .collection_id();
        self.data::<metadata::Data>()
            ._set_attribute(id, String::from("baseUri"), uri.into_bytes());
        Ok(())
    }

    /// Get URI for the token Id
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString, PSP34Error> {
        self._token_exists(Id::U64(token_id))?;
        let value = self.get_attribute(
            self.data::<psp34::Data<enumerable::Balances>>()
                .collection_id(),
            String::from("baseUri"),
        );
        let mut token_uri = PreludeString::from_utf8(value.unwrap()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &PreludeString::from(".json");
        Ok(token_uri)
    }

    /// Get max supply of tokens
    default fn max_supply(&self) -> u64 {
        self.data::<Data>().max_supply
    }

    /// Get token mint price
    default fn price(&self) -> Balance {
        self.data::<Data>().price_per_mint
    }

    /// Withdraw contract's balance
    #[modifiers(only_owner)]
    default fn withdraw(&mut self) -> Result<(), PSP34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        Self::env()
            .transfer(self.data::<ownable::Data>().owner(), current_balance)
            .map_err(|_| PSP34Error::Custom(String::from(RmrkError::WithdrawalFailed.as_str())))?;
        Ok(())
    }
}

/// Helper trait for Psp34Custom
impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if the transferred mint values is as expected
    default fn _check_value(
        &self,
        transfered_value: u128,
        mint_amount: u64,
    ) -> Result<(), PSP34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transfered_value == value {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::BadMintValue.as_str(),
        )));
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::CannotMintZeroTokens.as_str(),
            )));
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::CollectionIsFull.as_str(),
        )));
    }

    /// Check if token is minted
    default fn _token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}

//---------------------- T E S T ---------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_value_overflow_ok2() {
        struct testing {};
        impl Psp34Custom for testing {}
        assert_eq!(
            testing._check_value(transferred_value, mint_amount),
            Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
        );
    }
}
