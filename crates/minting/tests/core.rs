#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

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
        use super::*;
        use ink_env::{
            test,
            AccountId,
        };
        use ink_lang as ink;

        use ink_prelude::string::String as PreludeString;
        use rmrk_common::{
            errors::*,
            utils::Utils,
        };

        use openbrush::contracts::{
            ownable::OwnableError,
            psp34::PSP34Error,
        };

        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

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
            assert_eq!(rmrk.total_supply(), 1);
            assert_eq!(rmrk.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(rmrk.balance_of(accounts.bob), 1);
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(rmrk.minting.last_token_id, 1);
            assert_eq!(1, ink_env::test::recorded_events().count());
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

            assert_eq!(rmrk.total_supply(), num_of_mints as u128);
            assert_eq!(rmrk.balance_of(accounts.bob), 5);
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
            assert_eq!(5, ink_env::test::recorded_events().count());
            assert_eq!(
                rmrk.owners_token_by_index(accounts.bob, 5),
                Err(PSP34Error::TokenNotExists)
            );
        }

        #[ink::test]
        fn mint_with_metadata_works() {
            const RMRK_METADATA: &str = "ipfs://rmrkIpfsUri/";

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

            // token_uri for mint_next work
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(rmrk.mint(accounts.bob).is_ok());
            assert_eq!(
                rmrk.token_uri(3),
                Ok(PreludeString::from(BASE_URI.to_owned() + "3.json"))
            );
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            assert_eq!(rmrk.total_supply(), 0);
            assert_eq!(
                rmrk.mint_many(accounts.bob, num_of_mints),
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
            assert_eq!(
                rmrk.token_uri(1),
                Ok(PreludeString::from(BASE_URI.to_owned() + "1.json"))
            );

            // return error if request is for not yet minted token
            assert_eq!(rmrk.token_uri(42), Err(PSP34Error::TokenNotExists.into()));

            // verify token_uri when baseUri is empty
            set_sender(accounts.alice);
            assert!(rmrk.set_base_uri(PreludeString::from("")).is_ok());
            assert_eq!(
                rmrk.token_uri(1),
                Ok("".to_owned() + &PreludeString::from("1.json"))
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
    }
}
