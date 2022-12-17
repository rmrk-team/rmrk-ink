#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract {
    use ink_env;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::Vec;
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
    use rmrk::{
        impls::rmrk::{
            types::*,
            *,
        },
        traits::{
            base::*,
            minting::*,
            multiasset::*,
            nesting::*,
            utils::*,
        },
    };

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

    /// Event emitted when a new child is added.
    #[ink(event)]
    pub struct ChildAdded {
        #[ink(topic)]
        to: Id,
        #[ink(topic)]
        collection: AccountId,
        #[ink(topic)]
        child: Id,
    }

    /// Event emitted when a child is accepted.
    #[ink(event)]
    pub struct ChildAccepted {
        #[ink(topic)]
        parent: Id,
        #[ink(topic)]
        collection: AccountId,
        #[ink(topic)]
        child: Id,
    }

    /// Event emitted when a child is removed.
    #[ink(event)]
    pub struct ChildRemoved {
        #[ink(topic)]
        parent: Id,
        #[ink(topic)]
        child_collection: AccountId,
        #[ink(topic)]
        child_token_id: Id,
    }

    /// Event emitted when a child is rejected.
    #[ink(event)]
    pub struct ChildRejected {
        #[ink(topic)]
        parent: Id,
        #[ink(topic)]
        child_collection: AccountId,
        #[ink(topic)]
        child_token_id: Id,
    }

    /// Event emitted when new asset is set for the collection.
    #[ink(event)]
    pub struct AssetSet {
        #[ink(topic)]
        asset: AssetId,
    }
    /// Event emitted when the asset is added to the token.
    #[ink(event)]
    pub struct AssetAddedToToken {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
        #[ink(topic)]
        replaces: Option<Id>,
    }

    /// Event emitted when the asset is accepted.
    #[ink(event)]
    pub struct AssetAccepted {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
    }

    /// Event emitted when the asset is rejected.
    #[ink(event)]
    pub struct AssetRejected {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
    }

    /// Event emitted when the asset is removed.
    #[ink(event)]
    pub struct AssetRemoved {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
    }

    /// Event emitted when the asset is removed.
    #[ink(event)]
    pub struct AssetPrioritySet {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        priorities: Vec<AssetId>,
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
        utils: types::UtilsData,
        #[storage_field]
        nesting: types::NestingData,
        #[storage_field]
        multiasset: types::MultiAssetData,
        #[storage_field]
        minting: types::MintingData,
        #[storage_field]
        base: types::BaseData,
    }

    impl PSP34 for Rmrk {}

    impl Ownable for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Utils for Rmrk {}

    impl Minting for Rmrk {}

    impl Nesting for Rmrk {}

    impl MultiAsset for Rmrk {}

    impl Base for Rmrk {}

    impl Rmrk {
        /// Instantiate new RMRK contract
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
                    collection_id,
                    String::from("collection_metadata"),
                    collection_metadata,
                );
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

    impl nesting::NestingEvents for Rmrk {
        /// Emit ChildAdded event
        fn _emit_added_child_event(&self, to: &Id, collection: &AccountId, child: &Id) {
            self.env().emit_event(ChildAdded {
                to: to.clone(),
                collection: *collection,
                child: child.clone(),
            });
        }

        /// Emit ChildAccepted event
        fn _emit_child_accepted_event(&self, parent: &Id, collection: &AccountId, child: &Id) {
            self.env().emit_event(ChildAccepted {
                parent: parent.clone(),
                collection: *collection,
                child: child.clone(),
            });
        }

        /// Emit ChildRemoved event
        fn _emit_child_removed_event(
            &self,
            parent: &Id,
            child_collection: &AccountId,
            child_token_id: &Id,
        ) {
            self.env().emit_event(ChildRemoved {
                parent: parent.clone(),
                child_collection: *child_collection,
                child_token_id: child_token_id.clone(),
            });
        }

        /// Emit ChildRejected event
        fn _emit_child_rejected_event(
            &self,
            parent: &Id,
            child_collection: &AccountId,
            child_token_id: &Id,
        ) {
            self.env().emit_event(ChildRejected {
                parent: parent.clone(),
                child_collection: *child_collection,
                child_token_id: child_token_id.clone(),
            });
        }
    }
    impl multiasset::MultiAssetEvents for Rmrk {
        /// Used to notify listeners that an asset object is initialized at `assetId`.
        fn _emit_asset_set_event(&self, asset_id: &AssetId) {
            self.env().emit_event(AssetSet { asset: *asset_id });
        }

        /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
        fn _emit_asset_added_to_token_event(
            &self,
            token_id: &Id,
            asset_id: &AssetId,
            replaces_id: Option<Id>,
        ) {
            self.env().emit_event(AssetAddedToToken {
                token: token_id.clone(),
                asset: *asset_id,
                replaces: replaces_id,
            });
        }

        /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
        fn _emit_asset_accepted_event(&self, token_id: &Id, asset_id: &AssetId) {
            self.env().emit_event(AssetAccepted {
                token: token_id.clone(),
                asset: *asset_id,
            });
        }

        /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped from the pending assets array of the token.
        fn _emit_asset_rejected_event(&self, token_id: &Id, asset_id: &AssetId) {
            self.env().emit_event(AssetRejected {
                token: token_id.clone(),
                asset: *asset_id,
            });
        }

        /// Used to notify listeners that an asset object at `assetId` is removed from token
        fn _emit_asset_removed_event(&self, token_id: &Id, asset_id: &AssetId) {
            self.env().emit_event(AssetRemoved {
                token: token_id.clone(),
                asset: *asset_id,
            });
        }

        /// Used to notify listeners that token's prioritiy array is reordered.
        fn _emit_asset_priority_set_event(&self, token_id: &Id, priorities: Vec<AssetId>) {
            self.env().emit_event(AssetPrioritySet {
                token: token_id.clone(),
                priorities,
            });
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::rmrk_contract::PSP34Error::*;
        use ink_env::{
            pay_with_call,
            test,
        };
        use ink_lang as ink;
        use ink_prelude::string::String as PreludeString;
        use rmrk::impls::rmrk::{
            errors::RmrkError,
            minting::Internal,
        };

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
            test::set_value_transferred::<ink_env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(rmrk.mint(accounts.bob, num_of_mints).is_ok());
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
                rmrk.mint(accounts.bob, num_of_mints),
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
                rmrk.mint(accounts.bob, num_of_mints),
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
            rmrk.minting.last_token_id = max_supply - 1;

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

        #[ink::test]
        fn add_asset_entry_works() {
            const ASSET_URI: &str = "asset_uri/";
            const ASSET_ID: AssetId = 1;
            let mut rmrk = init();
            assert!(rmrk
                .add_asset_entry(ASSET_ID, 1, 1, String::from(ASSET_URI), vec![1, 2, 3],)
                .is_ok());
            assert_eq!(rmrk.total_assets(), 1);
            assert_eq!(rmrk.get_asset_uri(ASSET_ID), Some(String::from(ASSET_URI)));
            assert_eq!(rmrk.get_asset_uri(42), None);

            // reject adding asset with same asset_id
            assert_eq!(
                rmrk.add_asset_entry(ASSET_ID, 1, 1, String::from(ASSET_URI), vec![1, 2, 3],),
                Err(PSP34Error::Custom(RmrkError::AssetIdAlreadyExists.as_str()))
            );
        }

        #[ink::test]
        fn add_asset_to_token_works() {
            let accounts = default_accounts();
            const ASSET_URI: &str = "asset_uri/";
            const ASSET_ID: AssetId = 1;
            const TOKEN_ID1: Id = Id::U64(1);
            const TOKEN_ID2: Id = Id::U64(2);

            let mut rmrk = init();
            // Add new asset entry
            assert!(rmrk
                .add_asset_entry(ASSET_ID, 1, 1, String::from(ASSET_URI), vec![1, 2, 3],)
                .is_ok());
            assert_eq!(rmrk.total_assets(), 1);
            assert_eq!(1, ink_env::test::recorded_events().count());

            // mint token and add asset to it. Should be accepted without approval
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE as u128);
            assert!(rmrk.mint(accounts.alice, 1).is_ok());
            assert_eq!(2, ink_env::test::recorded_events().count());
            assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID, None).is_ok());
            assert_eq!(4, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID1), Ok((1, 0)));

            // error cases
            assert_eq!(
                rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID, None),
                Err(PSP34Error::Custom(RmrkError::AlreadyAddedAsset.as_str()))
            );
            assert_eq!(
                rmrk.add_asset_to_token(TOKEN_ID1, 42, None),
                Err(PSP34Error::Custom(RmrkError::AssetIdNotFound.as_str()))
            );

            // mint second token to non owner (Bob)
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE as u128);
            assert!(rmrk.mint(accounts.bob, 1).is_ok());
            assert_eq!(5, ink_env::test::recorded_events().count());

            // Add asset by alice and reject asset by Bob to test asset_reject
            assert!(rmrk.add_asset_to_token(TOKEN_ID2, ASSET_ID, None).is_ok());
            assert_eq!(6, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 1)));
            set_sender(accounts.bob);
            assert!(rmrk.reject_asset(TOKEN_ID2, ASSET_ID).is_ok());
            assert_eq!(7, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 0)));

            // Add asset by alice and accept asset by Bob, to test accept_asset
            set_sender(accounts.alice);
            assert!(rmrk.add_asset_to_token(TOKEN_ID2, ASSET_ID, None).is_ok());
            assert_eq!(8, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 1)));
            set_sender(accounts.bob);
            assert!(rmrk.accept_asset(TOKEN_ID2, ASSET_ID).is_ok());
            assert_eq!(9, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((1, 0)));
            assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID2), Ok(Some(vec![1])));

            // Try adding asset to not minted token
            assert_eq!(
                rmrk.add_asset_to_token(Id::U64(3), ASSET_ID, None),
                Err(TokenNotExists)
            );

            // Try removing not added asset
            assert_eq!(
                rmrk.remove_asset(TOKEN_ID2, 42),
                Err(PSP34Error::Custom(RmrkError::AssetIdNotFound.as_str()))
            );

            // Try removing asset for not minted token
            assert_eq!(rmrk.remove_asset(Id::U64(3), ASSET_ID), Err(TokenNotExists));

            // Remove accepted asset
            assert!(rmrk.remove_asset(TOKEN_ID2, ASSET_ID).is_ok());
            assert_eq!(10, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID2), Ok(Some(vec![])));
            assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 0)));
        }

        #[ink::test]
        fn set_asset_priority_works() {
            let accounts = default_accounts();
            const ASSET_URI: &str = "asset_uri/";
            const ASSET_ID1: AssetId = 1;
            const ASSET_ID2: AssetId = 100;
            const TOKEN_ID1: Id = Id::U64(1);

            let mut rmrk = init();
            // Add new asset entry
            assert!(rmrk
                .add_asset_entry(ASSET_ID1, 1, 1, String::from(ASSET_URI), vec![1, 2, 3],)
                .is_ok());
            assert!(rmrk
                .add_asset_entry(ASSET_ID2, 1, 1, String::from(ASSET_URI), vec![1, 2, 3],)
                .is_ok());
            assert_eq!(rmrk.total_assets(), 2);

            // mint token and add two assets to it. Should be accepted without approval
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * 2 as u128);
            assert!(rmrk.mint(accounts.alice, 2).is_ok());
            assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID1, None).is_ok());
            assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID2, None).is_ok());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID1), Ok((2, 0)));
            assert_eq!(
                rmrk.get_accepted_token_assets(TOKEN_ID1),
                Ok(Some(vec![ASSET_ID1, ASSET_ID2]))
            );
            assert!(rmrk
                .set_priority(TOKEN_ID1, vec![ASSET_ID2, ASSET_ID1])
                .is_ok());
            assert_eq!(
                rmrk.get_accepted_token_assets(TOKEN_ID1),
                Ok(Some(vec![ASSET_ID2, ASSET_ID1]))
            );

            // error cases
            assert_eq!(
                rmrk.set_priority(TOKEN_ID1, vec![ASSET_ID2]),
                Err(PSP34Error::Custom(RmrkError::BadPriorityLength.as_str()))
            );
            assert_eq!(
                rmrk.set_priority(TOKEN_ID1, vec![ASSET_ID2, 42]),
                Err(PSP34Error::Custom(RmrkError::AssetIdNotFound.as_str()))
            );
        }

        #[ink::test]
        fn add_parts_to_base_works() {
            let accounts = default_accounts();
            const ASSET_URI: &str = "asset_uri/";
            const ASSET_ID: AssetId = 1;
            const TOKEN_ID1: Id = Id::U64(1);
            const TOKEN_ID2: Id = Id::U64(2);
            let part_list = vec![
                // Background option 1
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    metadata_uri: String::from("ipfs://backgrounds/1.svg"),
                    is_equippable_by_all: false,
                },
                // Background option 2
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    metadata_uri: String::from("ipfs://backgrounds/2.svg"),
                    is_equippable_by_all: false,
                },
                // Head option 1
                Part {
                    part_type: PartType::Fixed,
                    z: 3,
                    equippable: vec![],
                    metadata_uri: String::from("ipfs://heads/1.svg"),
                    is_equippable_by_all: false,
                },
                // Head option 2
                Part {
                    part_type: PartType::Fixed,
                    z: 3,
                    equippable: vec![],
                    metadata_uri: String::from("ipfs://heads/2.svg"),
                    is_equippable_by_all: false,
                },
                // Body option 1
                Part {
                    part_type: PartType::Fixed,
                    z: 2,
                    equippable: vec![],
                    metadata_uri: String::from("ipfs://body/1.svg"),
                    is_equippable_by_all: false,
                },
                // Body option 2
                Part {
                    part_type: PartType::Fixed,
                    z: 2,
                    equippable: vec![],
                    metadata_uri: String::from("ipfs://body/2.svg"),
                    is_equippable_by_all: false,
                },
            ];

            let mut rmrk = init();
            // Add new asset entry
            assert!(rmrk.get_parts_count() == 0);
            assert!(rmrk.add_part_list(part_list.clone()).is_ok());
            assert_eq!(rmrk.get_parts_count(), part_list.len() as u32);
            assert_eq!(rmrk.get_base_metadata(), "");
            assert_eq!(rmrk.get_part(0).unwrap().z, part_list[0].z);
            assert_eq!(rmrk.get_part(0).unwrap().metadata_uri, part_list[0].metadata_uri);
            // assert_eq!(1, ink_env::test::recorded_events().count());
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
