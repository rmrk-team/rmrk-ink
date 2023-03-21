#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod common;

#[openbrush::contract]
pub mod rmrk_contract_minting {

    use ink::codegen::{
        EmitEvent,
        Env,
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
        traits::{
            Storage,
            String,
        },
    };

    use rmrk_common::roles::CONTRIBUTOR;
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
    #[derive(Default, Storage)]
    pub struct Rmrk {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        minting: MintingData,
    }

    impl PSP34 for Rmrk {}

    impl AccessControl for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Minting for Rmrk {}

    impl Rmrk {
        #[allow(clippy::too_many_arguments)]
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: Option<u64>,
        ) -> Self {
            let mut instance = Rmrk::default();
            instance._init_with_admin(instance.env().caller());
            instance._setup_role(CONTRIBUTOR, instance.env().caller());
            let collection_id = instance.collection_id();
            instance._set_attribute(collection_id.clone(), String::from("name"), name);
            instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
            instance._set_attribute(collection_id.clone(), String::from("baseUri"), base_uri);
            instance.minting.max_supply = max_supply;
            instance
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

        use ink::prelude::string::String as PreludeString;
        use openbrush::contracts::psp34::PSP34Error;

        use rmrk_common::{
            errors::*,
            roles::{
                ADMIN,
                CONTRIBUTOR,
            },
        };

        use rmrk_minting::traits::Minting;

        use openbrush::contracts::{
            access_control::*,
            psp34::extensions::enumerable::*,
        };

        use openbrush::traits::{
            AccountId,
            String,
        };

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
                Some(MAX_SUPPLY),
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            assert!(rmrk.has_role(ADMIN, accounts.alice));
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
            assert!(rmrk.has_role(ADMIN, accounts.alice));

            let mut id = rmrk.mint(accounts.bob).unwrap();
            set_sender(accounts.bob);

            // only owner is allowed to assign metadata
            assert_eq!(
                rmrk.assign_metadata(id, RMRK_METADATA.into()),
                Err(AccessControlError::MissingRole.into())
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
            assert_eq!(2, ink::env::test::recorded_events().count());

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
        fn mint_single_without_limit_works() {
            let mut rmrk = Rmrk::new(
                String::from("Rmrk Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                None,
            );

            let accounts = default_accounts();
            assert_eq!(rmrk.total_supply(), 0);
            (0..MAX_SUPPLY + 1).for_each(|_| {
                let _ = rmrk.mint(accounts.alice);
            });

            assert_eq!(rmrk._last_token_id(), MAX_SUPPLY + 1);
        }


        #[ink::test]
        fn mint_many_without_limit_works() {
            let mut rmrk = Rmrk::new(
                String::from("Rmrk Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                None,
            );

            let accounts = default_accounts();
            assert_eq!(rmrk.total_supply(), 0);
            let num_of_mints = MAX_SUPPLY + 42;
            assert_eq!(
                rmrk.mint_many(accounts.alice, num_of_mints),
                Ok((Id::U64(1), Id::U64(MAX_SUPPLY + 42)))
            );
        }

        #[ink::test]
        fn mint_as_contributor_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);

            assert_eq!(
                rmrk.mint(accounts.bob),
                Err(AccessControlError::MissingRole.into())
            );

            set_sender(accounts.alice);
            assert!(rmrk.grant_role(CONTRIBUTOR, accounts.bob).is_ok());

            set_sender(accounts.bob);
            assert!(rmrk.mint(accounts.bob).is_ok());
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
                .assign_metadata(Id::U64(1), String::from(RMRK_METADATA))
                .is_ok());

            assert_eq!(rmrk.token_uri(1), Ok(PreludeString::from(RMRK_METADATA)));
        }
    }
}
