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
        extensions::autoindex::*,
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
        #[storage_field]
        minting_autoindex: MintingAutoIndexData,
    }

    impl PSP34 for Rmrk {}

    impl AccessControl for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl MintingAutoIndex for Rmrk {}

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
        use super::Rmrk;
        use openbrush::{
            contracts::psp34::extensions::enumerable::*,
            traits::{
                AccountId,
                Balance,
            },
        };
        use rmrk_minting::extensions::autoindex::*;

        use crate::common::{
            check_mint_many_outcome,
            check_mint_single_outcome,
            default_accounts,
            Accessor,
            MAX_SUPPLY,
        };

        pub const PRICE: Balance = 100_000_000_000_000_000;

        impl Accessor for super::Rmrk {
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

        #[ink::test]
        fn autoindex_single_mint_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            assert_eq!(
                MintingAutoIndex::mint(&mut rmrk, accounts.bob),
                Ok(Id::U64(1))
            );
            check_mint_single_outcome(rmrk, accounts.bob, 1);
        }

        #[ink::test]
        fn autoindex_many_mint_works() {
            let mut rmrk = init();
            let accounts = default_accounts();
            assert_eq!(
                MintingAutoIndex::mint_many(&mut rmrk, accounts.bob, 5),
                Ok((Id::U64(1), Id::U64(5)))
            );

            check_mint_many_outcome(rmrk, accounts.bob, 5);
        }
    }
}
