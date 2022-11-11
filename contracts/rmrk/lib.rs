#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract {
    // imports from ink!
    use ink_env;
    use ink_lang::codegen::{EmitEvent, Env};
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    // imports from openbrush
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::{
                balances::Balances,
                extensions::{enumerable::*, metadata::*},
            },
            reentrancy_guard::*,
        },
        traits::Storage,
    };
    // local imports
    use rmrk::{impls::rmrk::*, traits::psp34_custom::*};

    // Event definition

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

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Rmrk {
        #[storage_field]
        psp34: psp34::Data<Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        psp34_custom: psp34_custom_types::Data,
    }

    // Section contains default implementation without any modifications
    impl Ownable for Rmrk {}
    impl PSP34Metadata for Rmrk {}
    // impl PSP34Enumerable for Rmrk {}

    // Rmrk specific implementations
    impl Psp34Custom for Rmrk {}

    impl Rmrk {
        #[ink(constructor, payable)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
            collection_metadata: String,
            _royalty_receiver: AccountId,
            _royalty: u8,
        ) -> Self {
            ink_env::debug_println!("####### initializing RMRK contract");
            ink_lang::codegen::initialize_contract(|_instance: &mut Rmrk| {
                _instance._init_with_owner(_instance.env().caller());
                let collection_id = _instance.collection_id();
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("name").into_bytes(),
                    name.into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("symbol").into_bytes(),
                    symbol.into_bytes(),
                );

                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("baseUri").into_bytes(),
                    base_uri.into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("collection_metadata").into_bytes(),
                    collection_metadata.into_bytes(),
                );
                _instance.psp34_custom.max_supply = max_supply;
                _instance.psp34_custom.price_per_mint = price_per_mint;
            })
        }
    }

    impl psp34_custom::Internal for Rmrk {
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
        use ink_env::test;
        use ink_lang as ink;

        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://ipfs/myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let rmrk_contract = init();
            let collection_id = rmrk_contract.collection_id();
            assert_eq!(
                rmrk_contract
                    .get_attribute(collection_id.clone(), String::from("name").into_bytes()),
                Some(String::from("Remark Project").into_bytes())
            );
            assert_eq!(
                rmrk_contract
                    .get_attribute(collection_id.clone(), String::from("symbol").into_bytes()),
                Some(String::from("RMK").into_bytes())
            );
            assert_eq!(
                rmrk_contract.get_attribute(collection_id, String::from("baseUri").into_bytes()),
                Some(String::from(BASE_URI).into_bytes())
            );
            assert_eq!(rmrk_contract.max_supply(), MAX_SUPPLY);
            assert_eq!(rmrk_contract.price(), PRICE);
        }

        fn init() -> Rmrk {
            let accounts = default_accounts();
            Rmrk::new(
                String::from("Remark Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                PRICE,
                String::from(BASE_URI),
                accounts.eve,
                0,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut rmrk_contract = init();
            let accounts = default_accounts();
            assert_eq!(rmrk_contract.owner(), accounts.alice);
            set_sender(accounts.bob);

            assert_eq!(rmrk_contract.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(rmrk_contract.mint_next().is_ok());
            assert_eq!(rmrk_contract.total_supply(), 1);
            assert_eq!(rmrk_contract.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(rmrk_contract.balance_of(accounts.bob), 1);
            // assert_eq!(
            //     rmrk_contract.owners_token_by_index(accounts.bob, 0),
            //     Ok(Id::U64(1))
            // );
            assert_eq!(rmrk_contract.psp34_custom.last_token_id, 1);
            assert_eq!(1, ink_env::test::recorded_events().count());
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut rmrk_contract = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = 5;

            assert_eq!(rmrk_contract.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(rmrk_contract.mint_for(accounts.bob, num_of_mints).is_ok());
            assert_eq!(rmrk_contract.total_supply(), num_of_mints as u128);
            assert_eq!(rmrk_contract.balance_of(accounts.bob), 5);
            // assert_eq!(rmrk_contract.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            // assert_eq!(rmrk_contract.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
            // assert_eq!(rmrk_contract.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
            // assert_eq!(rmrk_contract.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
            // assert_eq!(rmrk_contract.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
            assert_eq!(5, ink_env::test::recorded_events().count());
            // assert_eq!(
            //     rmrk_contract.owners_token_by_index(accounts.bob, 5),
            //     Err(TokenNotExists)
            // );
        }

        // #[ink::test]
        // fn mint_above_limit_fails() {
        //     let mut rmrk_contract = init();
        //     let accounts = default_accounts();
        //     set_sender(accounts.alice);
        //     let num_of_mints: u64 = MAX_SUPPLY + 1;

        //     assert_eq!(rmrk_contract.total_supply(), 0);
        //     test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128);
        //     assert_eq!(
        //         rmrk_contract.mint_for(accounts.bob, num_of_mints),
        //         Err(Custom("CollectionFullOrLocked".to_string()))
        //     );
        // }

        // #[ink::test]
        // fn mint_low_value_fails() {
        //     let mut rmrk_contract = init();
        //     let accounts = default_accounts();
        //     set_sender(accounts.bob);
        //     let num_of_mints = 1;

        //     assert_eq!(rmrk_contract.total_supply(), 0);
        //     test::set_value_transferred::<ink_env::DefaultEnvironment>(
        //         PRICE * num_of_mints as u128 - 1,
        //     );
        //     assert_eq!(
        //         rmrk_contract.mint_for(accounts.bob, num_of_mints),
        //         Err(Custom("BadMintValue".to_string()))
        //     );
        //     test::set_value_transferred::<ink_env::DefaultEnvironment>(
        //         PRICE * num_of_mints as u128 - 1,
        //     );
        //     assert_eq!(rmrk_contract.mint_next(), Err(Custom("BadMintValue".to_string())));
        //     assert_eq!(rmrk_contract.total_supply(), 0);
        // }

        // #[ink::test]
        // fn token_uri_works() {
        //     let mut rmrk_contract = init();
        //     let accounts = default_accounts();
        //     set_sender(accounts.alice);

        //     test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
        //     assert!(rmrk_contract.mint_next().is_ok());
        //     assert_eq!(
        //         rmrk_contract.token_uri(1),
        //         Ok(BASE_URI.to_owned() + &String::from("1.json"))
        //     );
        //     // return error if request is for not yet minted token
        //     assert_eq!(rmrk_contract.token_uri(42), Err(TokenNotExists));
        // }

        // #[ink::test]
        // fn owner_is_set() {
        //     let accounts = default_accounts();
        //     let rmrk_contract = init();
        //     assert_eq!(rmrk_contract.owner(), accounts.alice);
        // }

        // #[ink::test]
        // fn set_base_uri_works() {
        //     let accounts = default_accounts();
        //     const NEW_BASE_URI: &str = "new_uri/";
        //     let mut rmrk_contract = init();

        //     set_sender(accounts.alice);
        //     assert!(rmrk_contract.set_base_uri(NEW_BASE_URI.to_string()).is_ok());
        //     assert_eq!(
        //         rmrk_contract.get_attribute(Id::U8(0), String::from("baseUri").into_bytes()),
        //         Some(String::from(NEW_BASE_URI).into_bytes())
        //     );
        //     set_sender(accounts.bob);
        //     assert_eq!(
        //         rmrk_contract.set_base_uri("shallFail".to_string()),
        //         Err(Custom("O::CallerIsNotOwner".to_string()))
        //     );
        // }

        // #[ink::test]
        // fn check_supply_overflow_ok() {
        //     let max_supply = u64::MAX - 1;
        //     let mut rmrk_contract = Shiden34Contract::new(
        //         String::from("Shiden34"),
        //         String::from("rmrk_contract"),
        //         String::from(BASE_URI),
        //         max_supply,
        //         PRICE,
        //     );
        //     rmrk_contract.last_token_id = max_supply - 1;

        //     // check case when last_token_id.add(mint_amount) if more than u64::MAX
        //     assert_eq!(
        //         rmrk_contract._check_amount(3),
        //         Err(Custom("CollectionFullOrLocked".to_string()))
        //     );

        //     // check case when mint_amount is 0
        //     assert_eq!(
        //         rmrk_contract._check_amount(0),
        //         Err(Custom("CannotMintZeroTokens".to_string()))
        //     );
        // }

        // #[ink::test]
        // fn check_value_overflow_ok() {
        //     let max_supply = u64::MAX;
        //     let price = u128::MAX as u128;
        //     let rmrk_contract = Shiden34Contract::new(
        //         String::from("Shiden34"),
        //         String::from("rmrk_contract"),
        //         String::from(BASE_URI),
        //         max_supply,
        //         price,
        //     );
        //     let transferred_value = u128::MAX;
        //     let mint_amount = u64::MAX;
        //     assert_eq!(
        //         rmrk_contract._check_value(transferred_value, mint_amount),
        //         Err(Custom("BadMintValue".to_string()))
        //     );
        // }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
    }
}
