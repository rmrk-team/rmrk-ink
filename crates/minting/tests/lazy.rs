#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod common;

#[openbrush::contract]
pub mod rmrk_contract_minting {

    use openbrush::{
        contracts::{
            access_control::*,
            psp34::extensions::{
                enumerable::*,
                metadata::*,
            },
            reentrancy_guard::*,
        },
        traits::Storage,
    };

    use ink::codegen::{
        EmitEvent,
        Env,
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

    impl MintingLazy for Rmrk {}

    impl Rmrk {
        #[allow(clippy::too_many_arguments)]
        #[ink(constructor)]
        pub fn new(max_supply: Option<u64>, price_per_mint: Balance) -> Self {
            let mut instance = Rmrk::default();
            instance._init_with_admin(instance.env().caller());
            instance._setup_role(CONTRIBUTOR, instance.env().caller());
            instance.minting.max_supply = max_supply;
            instance.minting.price_per_mint = price_per_mint;
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
        use super::{
            Environment,
            Rmrk,
        };
        use ink::{
            codegen::Env,
            env::{
                pay_with_call,
                test,
            },
        };
        use openbrush::{
            contracts::{
                access_control::*,
                psp34::extensions::enumerable::*,
            },
            traits::{
                AccountId,
                Balance,
            },
        };
        use rmrk_common::{
            errors::RmrkError,
            roles::ADMIN,
            utils::Utils,
        };
        use rmrk_minting::traits::MintingLazy;

        use crate::common::{
            check_mint_many_outcome,
            check_mint_single_outcome,
            default_accounts,
            set_sender,
            Accessor,
            MAX_SUPPLY,
        };

        pub const PRICE: Balance = 100_000_000_000_000_000;

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

        fn init() -> Rmrk {
            Rmrk::new(Some(MAX_SUPPLY), PRICE)
        }

        fn purchase(amount: u64) {
            test::set_value_transferred::<ink::env::DefaultEnvironment>(PRICE * amount as u128);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }

        #[ink::test]
        fn init_with_price_works() {
            let rmrk = init();
            assert_eq!(rmrk.max_supply(), Some(MAX_SUPPLY));
            assert_eq!(rmrk.price(), PRICE);
        }

        #[ink::test]
        fn mint_low_value_fails() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let num_of_mints = 1;

            assert_eq!(rmrk.total_supply(), 0);
            purchase(num_of_mints - 1);
            assert_eq!(
                rmrk.mint_many(num_of_mints),
                Err(RmrkError::BadMintValue.into())
            );
            purchase(num_of_mints - 1);
            assert_eq!(rmrk.mint(), Err(RmrkError::BadMintValue.into()));
            assert_eq!(rmrk.total_supply(), 0);
        }

        #[ink::test]
        fn withdrawal_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_balance(accounts.bob, PRICE);
            set_sender(accounts.bob);

            assert!(pay_with_call!(rmrk.mint(), PRICE).is_ok());
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
        fn mint_single_lazy_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            assert!(rmrk.has_role(ADMIN, accounts.alice));
            assert_eq!(rmrk.total_supply(), 0);
            set_sender(accounts.bob);
            purchase(1);
            assert!(rmrk.mint().is_ok());
            check_mint_single_outcome(rmrk, accounts.bob, 1);
        }

        #[ink::test]
        fn mint_events_works() {
            let mut rmrk = init();
            let num_of_mints: u64 = 5;
            purchase(num_of_mints);
            assert!(rmrk.mint_many(num_of_mints).is_ok());
            assert_eq!(
                num_of_mints as usize,
                ink::env::test::recorded_events().count()
            );
        }

        #[ink::test]
        fn mint_many_lazy_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            let num_of_mints: u64 = 5;
            set_sender(accounts.bob);
            assert_eq!(rmrk.total_supply(), 0);
            purchase(num_of_mints);
            assert!(rmrk.mint_many(num_of_mints).is_ok());
            check_mint_many_outcome(rmrk, accounts.bob, num_of_mints);
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut rmrk = init();
            let num_of_mints: u64 = MAX_SUPPLY + 1;
            assert_eq!(rmrk.total_supply(), 0);
            assert_eq!(
                rmrk.mint_many(num_of_mints),
                Err(RmrkError::CollectionIsFull.into())
            );
        }

        #[ink::test]
        fn mint_single_lazy_without_limit_works() {
            let mut rmrk = Rmrk::new(None, PRICE);

            let accounts = default_accounts();
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            set_sender(accounts.bob);
            assert_eq!(rmrk.total_supply(), 0);

            (0..num_of_mints).for_each(|_| {
                purchase(1);
                assert!(rmrk.mint().is_ok());
            });

            check_mint_many_outcome(rmrk, accounts.bob, num_of_mints);
        }

        #[ink::test]
        fn mint_single_lazy_with_limit_set_to_zero_works() {
            let mut rmrk = Rmrk::new(Some(0), PRICE);

            let accounts = default_accounts();
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            set_sender(accounts.bob);
            assert_eq!(rmrk.total_supply(), 0);

            (0..num_of_mints).for_each(|_| {
                purchase(1);
                assert!(rmrk.mint().is_ok());
            });

            check_mint_many_outcome(rmrk, accounts.bob, num_of_mints);
        }

        #[ink::test]
        fn mint_many_lazy_without_limit_works() {
            let mut rmrk = Rmrk::new(None, PRICE);

            let accounts = default_accounts();
            let num_of_mints: u64 = MAX_SUPPLY + 42;

            set_sender(accounts.bob);
            assert_eq!(rmrk.total_supply(), 0);

            purchase(num_of_mints);
            assert!(rmrk.mint_many(num_of_mints).is_ok());
            check_mint_many_outcome(rmrk, accounts.bob, num_of_mints);
        }

        #[ink::test]
        fn mint_many_lazy_with_limit_set_to_zero_works() {
            let mut rmrk = Rmrk::new(Some(0), PRICE);

            let accounts = default_accounts();
            let num_of_mints: u64 = MAX_SUPPLY + 42;

            set_sender(accounts.bob);
            assert_eq!(rmrk.total_supply(), 0);

            purchase(num_of_mints);
            assert!(rmrk.mint_many(num_of_mints).is_ok());
            check_mint_many_outcome(rmrk, accounts.bob, num_of_mints);
        }
    }
}
