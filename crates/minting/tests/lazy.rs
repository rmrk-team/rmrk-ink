#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract_minting {

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
        traits::Storage,
    };

    use ink_lang::codegen::{
        EmitEvent,
        Env,
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

    impl MintingLazy for Rmrk {}

    impl Rmrk {
        #[allow(clippy::too_many_arguments)]
        #[ink(constructor)]
        pub fn new(max_supply: u64, price_per_mint: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Rmrk| {
                instance._init_with_owner(instance.env().caller());
                instance.minting.max_supply = max_supply;
                instance.minting.price_per_mint = price_per_mint;
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
        use super::{
            Environment,
            Rmrk,
        };
        use ink_env::{
            pay_with_call,
            test,
            AccountId,
        };
        use ink_lang as ink;
        use ink_lang::codegen::Env;
        use openbrush::{
            contracts::{
                ownable::*,
                psp34::extensions::enumerable::*,
            },
            traits::Balance,
        };
        use rmrk_common::{
            errors::RmrkError,
            utils::Utils,
        };
        use rmrk_minting::traits::MintingLazy;

        const PRICE: Balance = 100_000_000_000_000_000;
        const MAX_SUPPLY: u64 = 10;

        fn init() -> Rmrk {
            Rmrk::new(MAX_SUPPLY, PRICE)
        }

        #[ink::test]
        fn init_with_price_works() {
            let rmrk = init();
            assert_eq!(rmrk.max_supply(), MAX_SUPPLY);
            assert_eq!(rmrk.price(), PRICE);
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
                rmrk.mint_many(num_of_mints),
                Err(RmrkError::BadMintValue.into())
            );
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
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
            assert_eq!(rmrk.owner(), accounts.alice);
            assert_eq!(rmrk.total_supply(), 0);
            set_sender(accounts.bob);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(rmrk.mint().is_ok());
            assert_eq!(rmrk.total_supply(), 1);
            assert_eq!(rmrk.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(rmrk.balance_of(accounts.bob), 1);
            assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(rmrk.minting.last_token_id, 1);
            assert_eq!(1, ink_env::test::recorded_events().count());
        }

        #[ink::test]
        fn mint_multiple_lazy_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let num_of_mints: u64 = 5;

            assert_eq!(rmrk.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(rmrk.mint_many(num_of_mints).is_ok());
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
