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

#![cfg_attr(not(feature = "std"), no_std)]
// imports from ink!
use ink_env;
use ink_prelude::string::{String, ToString};

// imports from openbrush
use crate::impls::rmrk::psp34_custom_types::{Data, RmrkError};
pub use crate::traits::psp34_custom::PSP34Custom;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::{
            extensions::{enumerable::*, metadata::*},
            Internal,
        },
        reentrancy_guard::*,
    },
    modifiers,
    traits::{AccountId, Balance, Storage},
};

impl<T> PSP34Custom for T
where
    T: Storage<Data>
        + Storage<psp34::Data>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata,
{
    /// Mint next available token for the caller
    default fn mint_next(&mut self) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), 1)?;
        let caller = Self::env().caller();
        if let Some(token_id) = self.data::<Data>().last_token_id.checked_add(1) {
            let mint_result = self
                .data::<psp34::Data>()
                ._mint_to(caller, Id::U64(token_id));
            self.data::<Data>().last_token_id += 1;

            ink_env::debug_println!("####### minting mint_result: {:?}", mint_result);
            assert!(mint_result.is_ok());

            return Ok(());
        }
        return Err(PSP34Error::Custom(
            RmrkError::CollectionFullOrLocked.as_str(),
        ));
    }

    /// Mint several tokens
    #[modifiers(non_reentrant)]
    default fn mint_for(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        self._check_amount(mint_amount)?;

        let next_to_mint = self.data::<Data>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            assert!(self
                .data::<psp34::Data>()
                ._mint_to(to, Id::U64(mint_id))
                .is_ok());
            self.data::<Data>().last_token_id += 1;
        }

        Ok(())
    }

    /// Set new value for the baseUri
    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {
        self.data::<metadata::Data>()._set_attribute(
            Id::U8(0),
            String::from("baseUri").into_bytes(),
            uri.into_bytes(),
        );
        Ok(())
    }

    /// Get URI from token ID
    default fn token_uri(&self, token_id: u64) -> Result<String, PSP34Error> {
        _ = self._token_exists(Id::U64(token_id))?;
        let value = self.get_attribute(
            self.data::<psp34::Data>().collection_id(),
            String::from("baseUri").into_bytes(),
        );
        let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &String::from(".json");
        Ok(token_uri)
    }

    /// Get max supply of tokens
    default fn max_supply(&self) -> u64 {
        self.data::<Data>().max_supply
    }

    /// Get token price
    default fn price(&self) -> Balance {
        self.data::<Data>().price_per_mint
    }

    /// Get max supply of tokens
    #[modifiers(only_owner)]
    default fn withdraw(&mut self) -> Result<(), PSP34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        Self::env()
            .transfer(self.data::<ownable::Data>().owner(), current_balance)
            .map_err(|_| PSP34Error::Custom("WithdrawFailed".to_string()))?;
        Ok(())
    }

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

        return Err(PSP34Error::Custom("BadMintValue".to_string()));
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom("CannotMintZeroTokens".to_string()));
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom("CollectionFullOrLocked".to_string()));
    }

    /// Check if token is minted
    default fn _token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data>()
            .owner_of(id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;
    const PRICE: Balance = 100_000_000_000_000_000;
    const BASE_URI: &str = "ipfs://myIpfsUri/";
    const MAX_SUPPLY: u64 = 10;
    use crate::shiden34::PSP34Error::*;
    use ink_env::test;

    #[ink::test]
    fn init_works() {
        let sh34 = init();
        let collection_id = sh34.collection_id();
        assert_eq!(
            sh34.get_attribute(collection_id.clone(), String::from("name").into_bytes()),
            Some(String::from("Shiden34").into_bytes())
        );
        assert_eq!(
            sh34.get_attribute(collection_id.clone(), String::from("symbol").into_bytes()),
            Some(String::from("SH34").into_bytes())
        );
        assert_eq!(
            sh34.get_attribute(collection_id, String::from("baseUri").into_bytes()),
            Some(String::from(BASE_URI).into_bytes())
        );
        assert_eq!(sh34.max_supply, MAX_SUPPLY);
        assert_eq!(sh34.price_per_mint, PRICE);
    }

    fn init() -> Shiden34Contract {
        Shiden34Contract::new(
            String::from("Shiden34"),
            String::from("SH34"),
            String::from(BASE_URI),
            MAX_SUPPLY,
            PRICE,
        )
    }

    #[ink::test]
    fn mint_single_works() {
        let mut sh34 = init();
        let accounts = default_accounts();
        assert_eq!(sh34.owner(), accounts.alice);
        set_sender(accounts.bob);

        assert_eq!(sh34.total_supply(), 0);
        test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
        assert!(sh34.mint_next().is_ok());
        assert_eq!(sh34.total_supply(), 1);
        assert_eq!(sh34.owner_of(Id::U64(1)), Some(accounts.bob));
        assert_eq!(sh34.balance_of(accounts.bob), 1);
        assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
        assert_eq!(sh34.last_token_id, 1);
        assert_eq!(1, ink_env::test::recorded_events().count());
    }

    #[ink::test]
    fn mint_multiple_works() {
        let mut sh34 = init();
        let accounts = default_accounts();
        set_sender(accounts.alice);
        let num_of_mints: u64 = 5;

        assert_eq!(sh34.total_supply(), 0);
        test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128);
        assert!(sh34.mint_for(accounts.bob, num_of_mints).is_ok());
        assert_eq!(sh34.total_supply(), num_of_mints as u128);
        assert_eq!(sh34.balance_of(accounts.bob), 5);
        assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
        assert_eq!(sh34.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
        assert_eq!(sh34.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
        assert_eq!(sh34.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
        assert_eq!(sh34.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
        assert_eq!(5, ink_env::test::recorded_events().count());
        assert_eq!(
            sh34.owners_token_by_index(accounts.bob, 5),
            Err(TokenNotExists)
        );
    }

    #[ink::test]
    fn mint_above_limit_fails() {
        let mut sh34 = init();
        let accounts = default_accounts();
        set_sender(accounts.alice);
        let num_of_mints: u64 = MAX_SUPPLY + 1;

        assert_eq!(sh34.total_supply(), 0);
        test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128);
        assert_eq!(
            sh34.mint_for(accounts.bob, num_of_mints),
            Err(Custom("CollectionFullOrLocked".to_string()))
        );
    }

    #[ink::test]
    fn mint_low_value_fails() {
        let mut sh34 = init();
        let accounts = default_accounts();
        set_sender(accounts.bob);
        let num_of_mints = 1;

        assert_eq!(sh34.total_supply(), 0);
        test::set_value_transferred::<ink_env::DefaultEnvironment>(
            PRICE * num_of_mints as u128 - 1,
        );
        assert_eq!(
            sh34.mint_for(accounts.bob, num_of_mints),
            Err(Custom("BadMintValue".to_string()))
        );
        test::set_value_transferred::<ink_env::DefaultEnvironment>(
            PRICE * num_of_mints as u128 - 1,
        );
        assert_eq!(sh34.mint_next(), Err(Custom("BadMintValue".to_string())));
        assert_eq!(sh34.total_supply(), 0);
    }

    #[ink::test]
    fn token_uri_works() {
        let mut sh34 = init();
        let accounts = default_accounts();
        set_sender(accounts.alice);

        test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
        assert!(sh34.mint_next().is_ok());
        assert_eq!(
            sh34.token_uri(1),
            Ok(BASE_URI.to_owned() + &String::from("1.json"))
        );
        // return error if request is for not yet minted token
        assert_eq!(sh34.token_uri(42), Err(TokenNotExists));
    }

    #[ink::test]
    fn owner_is_set() {
        let accounts = default_accounts();
        let sh34 = init();
        assert_eq!(sh34.owner(), accounts.alice);
    }

    #[ink::test]
    fn set_base_uri_works() {
        let accounts = default_accounts();
        const NEW_BASE_URI: &str = "new_uri/";
        let mut sh34 = init();

        set_sender(accounts.alice);
        assert!(sh34.set_base_uri(NEW_BASE_URI.to_string()).is_ok());
        assert_eq!(
            sh34.get_attribute(Id::U8(0), String::from("baseUri").into_bytes()),
            Some(String::from(NEW_BASE_URI).into_bytes())
        );
        set_sender(accounts.bob);
        assert_eq!(
            sh34.set_base_uri("shallFail".to_string()),
            Err(Custom("O::CallerIsNotOwner".to_string()))
        );
    }

    #[ink::test]
    fn check_supply_overflow_ok() {
        let max_supply = u64::MAX - 1;
        let mut sh34 = Shiden34Contract::new(
            String::from("Shiden34"),
            String::from("SH34"),
            String::from(BASE_URI),
            max_supply,
            PRICE,
        );
        sh34.last_token_id = max_supply - 1;

        // check case when last_token_id.add(mint_amount) if more than u64::MAX
        assert_eq!(
            sh34._check_amount(3),
            Err(Custom("CollectionFullOrLocked".to_string()))
        );

        // check case when mint_amount is 0
        assert_eq!(
            sh34._check_amount(0),
            Err(Custom("CannotMintZeroTokens".to_string()))
        );
    }

    #[ink::test]
    fn check_value_overflow_ok() {
        let max_supply = u64::MAX;
        let price = u128::MAX as u128;
        let sh34 = Shiden34Contract::new(
            String::from("Shiden34"),
            String::from("SH34"),
            String::from(BASE_URI),
            max_supply,
            price,
        );
        let transferred_value = u128::MAX;
        let mint_amount = u64::MAX;
        assert_eq!(
            sh34._check_value(transferred_value, mint_amount),
            Err(Custom("BadMintValue".to_string()))
        );
    }

    fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
        test::default_accounts::<Environment>()
    }

    fn set_sender(sender: AccountId) {
        ink_env::test::set_caller::<Environment>(sender);
    }
}
