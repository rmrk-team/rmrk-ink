//! Set of functions commonly used with PSP34 contract

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

use ink_prelude::string::{
    String as PreludeString,
    ToString,
};

use crate::impls::rmrk::{
    errors::RmrkError,
    types::MintingData,
};
pub use crate::traits::utils::{
    Internal,
    Utils,
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
        Balance,
        Storage,
        String,
    },
};

impl<T> Utils for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
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
        self.data::<MintingData>().max_supply
    }

    /// Get token mint price
    default fn price(&self) -> Balance {
        self.data::<MintingData>().price_per_mint
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
    T: Storage<MintingData> + Storage<psp34::Data<enumerable::Balances>>,
{
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
