#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract {
    use ink_env;
    use ink_lang::codegen::{EmitEvent, Env};
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{enumerable::*, metadata::*},
            reentrancy_guard::*,
        },
        traits::{Storage, String},
    };
    use psp34_helper::{impls::rmrk::*, traits::psp34_custom::*};

    // Event definitions
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
        psp34_custom: psp34_custom_types::Data,
    }

    impl PSP34 for Rmrk {}

    impl Ownable for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Psp34Custom for Rmrk {}

    impl Rmrk {
        #[ink(constructor)]
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
            ink_lang::codegen::initialize_contract(|instance: &mut Rmrk| {
                instance._init_with_owner(instance.env().caller());
                let collection_id = instance.collection_id();
                instance._set_attribute(collection_id.clone(), String::from("name"), name);
                instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
                instance._set_attribute(collection_id.clone(), String::from("baseUri"), base_uri);
                instance._set_attribute(
                    collection_id.clone(),
                    String::from("collection_metadata"),
                    collection_metadata,
                );
                instance.psp34_custom.max_supply = max_supply;
                instance.psp34_custom.price_per_mint = price_per_mint;
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
        use crate::rmrk_contract::PSP34Error::*;
        use ink_env::{pay_with_call, test};
        use ink_lang as ink;
        use ink_prelude::string::String as PreludeString;
        use psp34_helper::impls::rmrk::{psp34_custom::Internal, psp34_custom_types::RmrkError};
        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let rmrk = init();
            let collection_id = rmrk.collection_id();
            assert_eq!(
                rmrk.get_attribute(collection_id.clone(), String::from("name")),
                Some(String::from("Rmrk Project"))
            );
            assert_eq!(
                rmrk.get_attribute(collection_id.clone(), String::from("symbol")),
                Some(String::from("RMK"))
            );
            assert_eq!(
                rmrk.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(BASE_URI))
            );
            assert_eq!(rmrk.max_supply(), MAX_SUPPLY);
            assert_eq!(rmrk.price(), PRICE);
        }

        fn init() -> Rmrk {
            let accounts = default_accounts();
            Rmrk::new(
                String::from("Rmrk Project"),
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
            let mut rmrk = init();
            let accounts = default_accounts();
            assert_eq!(rmrk.owner(), accounts.alice);
            set_sender(accounts.bob);

            assert_eq!(rmrk.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(rmrk.mint_next().is_ok());
            assert_eq!(rmrk.total_supply(), 1);
            assert_eq!(rmrk.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(rmrk.balance_of(accounts.bob), 1);

            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(rmrk.psp34_custom.last_token_id, 1);
            assert_eq!(1, ink_env::test::recorded_events().count());
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = 5;

            assert_eq!(rmrk.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(rmrk.mint_for(accounts.bob, num_of_mints).is_ok());
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
                Err(TokenNotExists)
            );
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            assert_eq!(rmrk.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert_eq!(
                rmrk.mint_for(accounts.bob, num_of_mints),
                Err(PSP34Error::Custom(RmrkError::CollectionIsFull.as_str()))
            );
        }

        #[ink::test]
        fn mint_low_value_fails() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let num_of_mints = 1;

            assert_eq!(rmrk.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                rmrk.mint_for(accounts.bob, num_of_mints),
                Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
            );
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                rmrk.mint_next(),
                Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
            );
            assert_eq!(rmrk.total_supply(), 0);
        }

        #[ink::test]
        fn withdrawal_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_balance(accounts.bob, PRICE);
            set_sender(accounts.bob);

            assert!(pay_with_call!(rmrk.mint_next(), PRICE).is_ok());
            let expected_contract_balance = PRICE + rmrk.env().minimum_balance();
            assert_eq!(rmrk.env().balance(), expected_contract_balance);

            // Bob fails to withdraw
            set_sender(accounts.bob);
            assert!(rmrk.withdraw().is_err());
            assert_eq!(rmrk.env().balance(), expected_contract_balance);

            // Alice (contract owner) withdraws. Existential minimum is still set
            set_sender(accounts.alice);
            assert!(rmrk.withdraw().is_ok());
            // assert_eq!(rmrk.env().balance(), rmrk.env().minimum_balance());
        }

        #[ink::test]
        fn token_uri_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);

            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(rmrk.mint_next().is_ok());
            // return error if request is for not yet minted token
            assert_eq!(rmrk.token_uri(42), Err(TokenNotExists));
            assert_eq!(
                rmrk.token_uri(1),
                Ok(PreludeString::from(BASE_URI.to_owned() + "1.json"))
            );

            // return error if request is for not yet minted token
            assert_eq!(rmrk.token_uri(42), Err(TokenNotExists));

            // verify token_uri when baseUri is empty
            set_sender(accounts.alice);
            assert!(rmrk.set_base_uri(PreludeString::from("")).is_ok());
            assert_eq!(
                rmrk.token_uri(1),
                Ok("".to_owned() + &PreludeString::from("1.json"))
            );
        }

        #[ink::test]
        fn owner_is_set() {
            let accounts = default_accounts();
            let rmrk = init();
            assert_eq!(rmrk.owner(), accounts.alice);
        }

        #[ink::test]
        fn set_base_uri_works() {
            let accounts = default_accounts();
            const NEW_BASE_URI: &str = "new_uri/";
            let mut rmrk = init();

            set_sender(accounts.alice);
            let collection_id = rmrk.collection_id();
            assert!(rmrk.set_base_uri(NEW_BASE_URI.into()).is_ok());
            assert_eq!(
                rmrk.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(NEW_BASE_URI))
            );
            set_sender(accounts.bob);
            assert_eq!(
                rmrk.set_base_uri(NEW_BASE_URI.into()),
                Err(PSP34Error::Custom(String::from("O::CallerIsNotOwner")))
            );
        }

        #[ink::test]
        fn check_supply_overflow_ok() {
            let accounts = default_accounts();
            let max_supply = u64::MAX - 1;
            let mut rmrk = Rmrk::new(
                String::from("Remark Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                max_supply,
                PRICE,
                String::from(BASE_URI),
                accounts.eve,
                0,
            );
            rmrk.psp34_custom.last_token_id = max_supply - 1;

            // check case when last_token_id.add(mint_amount) if more than u64::MAX
            assert_eq!(
                rmrk._check_amount(3),
                Err(PSP34Error::Custom(RmrkError::CollectionIsFull.as_str()))
            );

            // check case when mint_amount is 0
            assert_eq!(
                rmrk._check_amount(0),
                Err(PSP34Error::Custom(RmrkError::CannotMintZeroTokens.as_str()))
            );
        }

        #[ink::test]
        fn check_value_overflow_ok() {
            let accounts = default_accounts();
            let max_supply = u64::MAX;
            let price = u128::MAX as u128;
            let rmrk = Rmrk::new(
                String::from("Remark Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                max_supply,
                price,
                String::from(BASE_URI),
                accounts.eve,
                0,
            );
            let transferred_value = u128::MAX;
            let mint_amount = u64::MAX;
            assert_eq!(
                rmrk._check_value(transferred_value, mint_amount),
                Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
        }
    }
}
