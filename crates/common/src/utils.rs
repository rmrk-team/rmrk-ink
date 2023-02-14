//! Set of functions commonly used with PSP34 contract

use ink_prelude::string::String as PreludeString;

use crate::{
    errors::{
        Result,
        RmrkError,
    },
    roles::{
        ADMIN,
        CONTRIBUTOR,
    },
};

use openbrush::{
    contracts::{
        access_control::*,
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

/// Trait definitions for Utils functions
#[openbrush::trait_definition]
pub trait Utils {
    /// Set new value for the baseUri.
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<()>;

    /// Withdraw contract's balance.
    #[ink(message)]
    fn withdraw(&mut self) -> Result<()>;

    /// Ensure that token exists
    fn ensure_exists_and_get_owner(&self, id: &Id) -> Result<AccountId>;

    /// Ensure that the caller is the token owner
    fn ensure_token_owner(&self, token_owner: AccountId) -> Result<()>;
}

impl<T> Utils for T
where
    T: Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<access_control::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    /// Set new value for the baseUri
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<()> {
        let id = self
            .data::<psp34::Data<enumerable::Balances>>()
            .collection_id();
        self.data::<metadata::Data>()
            ._set_attribute(id, String::from("baseUri"), uri.into_bytes());
        Ok(())
    }

    /// Withdraw contract's balance
    #[modifiers(only_role(ADMIN))]
    default fn withdraw(&mut self) -> Result<()> {
        let balance = Self::env().balance();
        let caller = Self::env().caller();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        Self::env()
            .transfer(caller, current_balance)
            .map_err(|_| RmrkError::WithdrawalFailed)?;
        Ok(())
    }
    /// Check if token is minted. Return the owner
    default fn ensure_exists_and_get_owner(&self, id: &Id) -> Result<AccountId> {
        let token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Ensure that the caller is the token owner
    default fn ensure_token_owner(&self, token_owner: AccountId) -> Result<()> {
        let caller = Self::env().caller();
        if caller != token_owner {
            return Err(RmrkError::NotTokenOwner.into())
        }
        Ok(())
    }
}
