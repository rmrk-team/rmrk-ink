#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod common;

#[openbrush::contract]
pub mod rmrk_contract_minting {

    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{
                enumerable::*,
                metadata::*,
            },
            reentrancy_guard::*,
        },
        traits::{
            Storage,
            String,
        },
    };
    use rmrk_minting::{
        traits::*,
        MintingData,
    };

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    // Rmrk contract storage
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Rmrk {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        minting: MintingData,
    }

    impl PSP34 for Rmrk {}

    impl Ownable for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Minting for Rmrk {}

    impl Rmrk {
        #[allow(clippy::too_many_arguments)]
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, base_uri: String, max_supply: u64) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Rmrk| {
                instance._init_with_owner(instance.env().caller());
                let collection_id = instance.collection_id();
                instance._set_attribute(collection_id.clone(), String::from("name"), name);
                instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
                instance._set_attribute(collection_id.clone(), String::from("baseUri"), base_uri);
                instance.minting.max_supply = max_supply;
            })
        }
    }

    impl psp34::Internal for Rmrk {
        /// Emit Transfer event
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        /// Emit Approval event
        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Rmrk;

        use ink_env::AccountId;
        use ink_lang as ink;

        use ink_prelude::string::String as PreludeString;
        use openbrush::contracts::psp34::PSP34Error;
        use rmrk_common::errors::*;
        use rmrk_minting::traits::Minting;

        use openbrush::contracts::{
            ownable::*,
            psp34::extensions::enumerable::*,
        };

        use openbrush::traits::String;

        use crate::common::{
            check_mint_many_outcome,
            check_mint_single_outcome,
            default_accounts,
            set_sender,
            Accessor,
            MAX_SUPPLY,
        };

        impl Accessor for super::Rmrk {
            fn _last_token_id(&self) -> u64 {
                self.minting.last_token_id
            }

            fn _owners_token_by_index(
                &self,
                account: AccountId,
                index: u128,
            ) -> core::result::Result<Id, PSP34Error> {
                self.owners_token_by_index(account, index)
            }
        }

        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const RMRK_METADATA: &str = "ipfs://rmrkIpfsUri/";

        fn init() -> Rmrk {
            Rmrk::new(
                String::from("Rmrk Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                MAX_SUPPLY,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            assert_eq!(rmrk.owner(), accounts.alice);
            assert_eq!(rmrk.total_supply(), 0);
            assert_eq!(rmrk.mint(accounts.bob), Ok(Id::U64(1)));
            check_mint_single_outcome(rmrk, accounts.bob, 1);
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = 5;

            assert_eq!(rmrk.total_supply(), 0);
            assert_eq!(
                rmrk.mint_many(accounts.bob, num_of_mints),
                Ok((Id::U64(1), Id::U64(5)))
            );
            check_mint_many_outcome(rmrk, accounts.bob, num_of_mints);
        }

        #[ink::test]
        fn mint_with_metadata_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            assert_eq!(rmrk.owner(), accounts.alice);

            let mut id = rmrk.mint(accounts.bob).unwrap();
            set_sender(accounts.bob);

            // only owner is allowed to assign metadata
            assert_eq!(
                rmrk.assign_metadata(id, RMRK_METADATA.into()),
                Err(OwnableError::CallerIsNotOwner.into())
            );

            // owner mints
            set_sender(accounts.alice);
            assert_eq!(rmrk.total_supply(), 1);
            id = rmrk.mint(accounts.alice).unwrap();

            assert!(rmrk.assign_metadata(id, RMRK_METADATA.into()).is_ok());
            assert_eq!(rmrk.total_supply(), 2);
            assert_eq!(rmrk.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(rmrk.balance_of(accounts.bob), 1);
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(2, ink_env::test::recorded_events().count());

            // token_uri for rmrk mint works
            assert_eq!(
                rmrk.token_uri(2),
                Ok(PreludeString::from(RMRK_METADATA.to_owned()))
            );
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut rmrk = init();
            let accounts = default_accounts();
            let num_of_mints: u64 = MAX_SUPPLY + 1;
            assert_eq!(rmrk.total_supply(), 0);
            assert_eq!(
                rmrk.mint_many(accounts.alice, num_of_mints),
                Err(RmrkError::CollectionIsFull.into())
            );
        }

        #[ink::test]
        fn token_uri_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);

            assert!(rmrk.mint(accounts.alice).is_ok());
            // return error if request is for not yet minted token
            assert_eq!(rmrk.token_uri(42), Err(PSP34Error::TokenNotExists.into()));
            // return error if metadata is net yet assigned
            assert_eq!(rmrk.token_uri(1), Err(RmrkError::UriNotFound.into()));

            assert!(rmrk
                .assign_metadata(Id::U64(1), PreludeString::from(RMRK_METADATA))
                .is_ok());

            assert_eq!(rmrk.token_uri(1), Ok(PreludeString::from(RMRK_METADATA)));
        }
    }
}
