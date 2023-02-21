#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_example_equippable {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
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

    use rmrk::{
        config,
        storage::*,
        traits::*,
        types::*,
    };

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
        replaces: Option<AssetId>,
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

    /// Event emitted when the asset is equipped.
    #[ink(event)]
    pub struct AssetEquipped {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
        #[ink(topic)]
        child: Id,
        #[ink(topic)]
        child_asset: AssetId,
    }

    /// Event emitted when the asset is un-equipped.
    #[ink(event)]
    pub struct AssetUnEquipped {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
        #[ink(topic)]
        slot: SlotId,
    }

    /// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
    /// equippable into a given slot and parent
    #[ink(event)]
    pub struct ParentEquippableGroupSet {
        #[ink(topic)]
        group: EquippableGroupId,
        #[ink(topic)]
        slot: SlotId,
        #[ink(topic)]
        parent: AccountId,
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
        access: access_control::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        nesting: NestingData,
        #[storage_field]
        multiasset: MultiAssetData,
        #[storage_field]
        minting: MintingData,
        #[storage_field]
        base: BaseData,
        #[storage_field]
        equippable: EquippableData,
    }

    impl PSP34 for Rmrk {}

    impl AccessControl for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Minting for Rmrk {}

    impl Nesting for Rmrk {}

    impl MultiAsset for Rmrk {}

    impl Base for Rmrk {}

    impl Equippable for Rmrk {}

    impl Rmrk {
        /// Instantiate new RMRK contract
        #[allow(clippy::too_many_arguments)]
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            collection_metadata: String,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Rmrk| {
                config::with_admin(instance, Self::env().caller());
                config::with_collection(
                    instance,
                    name,
                    symbol,
                    base_uri,
                    collection_metadata,
                    max_supply,
                );
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

    impl NestingEvents for Rmrk {
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

    impl MultiAssetEvents for Rmrk {
        /// Used to notify listeners that an asset object is initialized at `assetId`.
        fn _emit_asset_set_event(&self, asset_id: &AssetId) {
            self.env().emit_event(AssetSet { asset: *asset_id });
        }

        /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
        fn _emit_asset_added_to_token_event(
            &self,
            token_id: &Id,
            asset_id: &AssetId,
            replaces_id: &Option<AssetId>,
        ) {
            self.env().emit_event(AssetAddedToToken {
                token: token_id.clone(),
                asset: *asset_id,
                replaces: *replaces_id,
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

    impl EquippableEvents for Rmrk {
        /// Used to notify listeners that a child's asset has been equipped into one of its parent assets.
        fn emit_child_asset_equipped(
            &self,
            token_id: Id,
            asset_id: AssetId,
            _slot_part_id: PartId,
            child_nft: ChildNft,
            child_asset_id: AssetId,
        ) {
            self.env().emit_event(AssetEquipped {
                token: token_id,
                asset: asset_id,
                child: child_nft.1,
                child_asset: child_asset_id,
            });
        }

        /// Used to notify listeners that a child's asset has been un-equipped from one of its parent assets.
        fn emit_child_asset_unequipped(
            &self,
            token_id: Id,
            asset_id: AssetId,
            slot_part_id: PartId,
        ) {
            self.env().emit_event(AssetUnEquipped {
                token: token_id,
                asset: asset_id,
                slot: slot_part_id,
            });
        }

        /// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
        /// equippable into a given slot and parent
        fn emit_valid_parent_equippable_group_set(
            &self,
            group_id: EquippableGroupId,
            slot_part_id: PartId,
            parent_address: AccountId,
        ) {
            self.env().emit_event(ParentEquippableGroupSet {
                group: group_id,
                slot: slot_part_id,
                parent: parent_address,
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{
            Environment,
            Rmrk,
        };
        use crate::rmrk_example_equippable::PSP34Error::*;

        use openbrush::{
            contracts::{
                access_control::*,
                psp34::extensions::{
                    enumerable::*,
                    metadata::*,
                },
            },
            traits::String,
        };

        use ink_env::{
            test,
            AccountId,
        };

        use openbrush::contracts::{
            access_control::AccessControlError::*,
            psp34::PSP34Error,
        };

        use ink_lang as ink;

        use rmrk::{
            errors::*,
            roles::ADMIN,
            traits::{
                Base,
                Equippable,
                Minting,
                MultiAsset,
            },
            types::*,
            utils::Utils,
        };

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
        }

        fn init() -> Rmrk {
            Rmrk::new(
                String::from("Rmrk Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                String::from(BASE_URI),
            )
        }

        #[ink::test]
        fn owner_is_set() {
            let accounts = default_accounts();
            let rmrk = init();
            assert!(rmrk.has_role(ADMIN, accounts.alice));
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
                Err(MissingRole.into())
            );
        }

        #[ink::test]
        fn add_asset_entry_works() {
            const ASSET_URI1: &str = "asset_uri/";
            const ASSET_ID1: AssetId = 1;
            const ASSET_URI2: &str = "asset_uri2/";
            const ASSET_ID2: AssetId = 2;
            const EQUIPPABLE_GROUP_ID: EquippableGroupId = 1;

            let mut rmrk = init();
            assert!(rmrk
                .add_asset_entry(
                    ASSET_ID1,
                    EQUIPPABLE_GROUP_ID,
                    String::from(ASSET_URI1),
                    vec![]
                )
                .is_ok());
            assert_eq!(1, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_assets(), 1);
            assert_eq!(
                rmrk.get_asset_uri(ASSET_ID1),
                Some(String::from(ASSET_URI1))
            );
            assert_eq!(rmrk.get_asset_uri(42), None);

            // reject adding asset with same asset_id
            assert_eq!(
                rmrk.add_asset_entry(ASSET_ID1, 1, String::from(ASSET_URI1), vec![]),
                Err(RmrkError::AssetIdAlreadyExists.into())
            );

            // add one more asset
            assert!(rmrk
                .add_asset_entry(ASSET_ID2, 0, String::from(ASSET_URI2), vec![])
                .is_ok());
            assert_eq!(rmrk.total_assets(), 2);
            assert_eq!(
                rmrk.get_asset_uri(ASSET_ID2),
                Some(String::from(ASSET_URI2))
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
                .add_asset_entry(ASSET_ID, 1, String::from(ASSET_URI), vec![])
                .is_ok());
            assert_eq!(rmrk.total_assets(), 1);
            assert_eq!(1, ink_env::test::recorded_events().count());

            // mint token and add asset to it. Should be accepted without approval
            assert!(rmrk.mint(accounts.alice).is_ok());
            assert_eq!(2, ink_env::test::recorded_events().count());
            assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID, None).is_ok());
            assert_eq!(4, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID1), Ok((1, 0)));

            // error cases
            assert_eq!(
                rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID, None),
                Err(RmrkError::AlreadyAddedAsset.into())
            );
            assert_eq!(
                rmrk.add_asset_to_token(TOKEN_ID1, 42, None),
                Err(RmrkError::AssetIdNotFound.into())
            );

            // mint second token to non owner (Bob)
            set_sender(accounts.alice);
            assert!(rmrk.mint(accounts.bob).is_ok());
            assert_eq!(5, ink_env::test::recorded_events().count());

            // Add asset by alice and reject asset by Bob to test asset_reject
            set_sender(accounts.alice);
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
            assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID2), Ok(vec![1]));

            // Try adding asset to not minted token fails
            set_sender(accounts.alice);
            assert_eq!(
                rmrk.add_asset_to_token(Id::U64(3), ASSET_ID, None),
                Err(TokenNotExists.into())
            );

            // Try removing not added asset fails
            assert_eq!(
                rmrk.remove_asset(TOKEN_ID2, 42),
                Err(RmrkError::AssetIdNotFound.into())
            );

            // Try removing asset for not minted token fails
            assert_eq!(
                rmrk.remove_asset(Id::U64(3), ASSET_ID),
                Err(TokenNotExists.into())
            );

            // Try removing asset by collection owner fails
            set_sender(accounts.alice);
            assert_eq!(
                rmrk.remove_asset(TOKEN_ID2, ASSET_ID),
                Err(RmrkError::NotTokenOwner.into())
            );

            // Remove accepted asset
            set_sender(accounts.bob);
            assert!(rmrk.remove_asset(TOKEN_ID2, ASSET_ID).is_ok());
            assert_eq!(10, ink_env::test::recorded_events().count());
            assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID2), Ok(vec![]));
            assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 0)));
        }

        #[ink::test]
        fn add_asset_to_token_with_replace_works() {
            let accounts = default_accounts();
            const ASSET_URI1: &str = "asset_uri/1";
            const ASSET_URI2: &str = "asset_uri/2";
            const ASSET_URI3: &str = "asset_uri/3";
            const ASSET_ID1: AssetId = 1;
            const ASSET_ID2: AssetId = 2;
            const ASSET_ID3: AssetId = 3;
            const TOKEN_ID: Id = Id::U64(1);

            let mut rmrk = init();
            // Add new asset entry
            assert!(rmrk
                .add_asset_entry(ASSET_ID1, 0, String::from(ASSET_URI1), vec![])
                .is_ok());
            assert!(rmrk
                .add_asset_entry(ASSET_ID2, 0, String::from(ASSET_URI2), vec![])
                .is_ok());
            assert!(rmrk
                .add_asset_entry(ASSET_ID3, 0, String::from(ASSET_URI3), vec![])
                .is_ok());

            assert_eq!(rmrk.total_assets(), 3);

            // mint token and add asset to it. Should be accepted without approval
            assert!(rmrk.mint(accounts.alice).is_ok());

            assert_eq!(
                rmrk.add_asset_to_token(TOKEN_ID, ASSET_ID3, Some(ASSET_ID1)),
                Err(RmrkError::AcceptedAssetsMissing.into())
            );

            assert!(rmrk.add_asset_to_token(TOKEN_ID, ASSET_ID1, None).is_ok());
            assert!(rmrk.add_asset_to_token(TOKEN_ID, ASSET_ID2, None).is_ok());

            assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID), Ok(vec![1, 2]));
            // replace previously accepted ASSET_ID1 with ASSET_ID3
            assert!(rmrk
                .add_asset_to_token(TOKEN_ID, ASSET_ID3, Some(ASSET_ID1))
                .is_ok());
            assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID), Ok(vec![3, 2]));
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
                .add_asset_entry(ASSET_ID1, 1, String::from(ASSET_URI), vec![])
                .is_ok());
            assert!(rmrk
                .add_asset_entry(ASSET_ID2, 1, String::from(ASSET_URI), vec![])
                .is_ok());
            assert_eq!(rmrk.total_assets(), 2);

            // mint token and add two assets to it. Should be accepted without approval
            assert!(rmrk.mint_many(accounts.alice, 2).is_ok());
            assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID1, None).is_ok());
            assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID2, None).is_ok());
            assert_eq!(rmrk.total_token_assets(TOKEN_ID1), Ok((2, 0)));
            assert_eq!(
                rmrk.get_accepted_token_assets(TOKEN_ID1),
                Ok(vec![ASSET_ID1, ASSET_ID2])
            );
            assert!(rmrk
                .set_priority(TOKEN_ID1, vec![ASSET_ID2, ASSET_ID1])
                .is_ok());
            assert_eq!(
                rmrk.get_accepted_token_assets(TOKEN_ID1),
                Ok(vec![ASSET_ID2, ASSET_ID1])
            );

            // error cases
            assert_eq!(
                rmrk.set_priority(TOKEN_ID1, vec![ASSET_ID2]),
                Err(RmrkError::BadPriorityLength.into())
            );
            assert_eq!(
                rmrk.set_priority(TOKEN_ID1, vec![ASSET_ID2, 42]),
                Err(RmrkError::AssetIdNotFound.into())
            );
        }

        #[ink::test]
        fn add_parts_to_base_works() {
            const ASSET_URI: &str = "asset_uri/";
            const ASSET_ID: AssetId = 1;
            const TOKEN_ID1: Id = Id::U64(1);
            const TOKEN_ID2: Id = Id::U64(2);
            const EQUIPABLE_ADDRESS1: [u8; 32] = [1; 32];
            const EQUIPABLE_ADDRESS2: [u8; 32] = [2; 32];
            const EQUIPABLE_ADDRESS3: [u8; 32] = [3; 32];
            const PART_ID0: PartId = 0;
            const PART_ID1: PartId = 1;
            let part_list = vec![
                // Background option 1
                Part {
                    part_type: PartType::Slot,
                    z: 0,
                    equippable: vec![EQUIPABLE_ADDRESS1.into(), EQUIPABLE_ADDRESS2.into()],
                    part_uri: String::from("ipfs://backgrounds/1.svg"),
                    is_equippable_by_all: false,
                },
                // Background option 2
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    part_uri: String::from("ipfs://backgrounds/2.svg"),
                    is_equippable_by_all: false,
                },
            ];

            let bad_part_list1 = vec![Part {
                part_type: PartType::Fixed,
                z: 0,
                equippable: vec![EQUIPABLE_ADDRESS1.into()],
                part_uri: String::from("ipfs://backgrounds/2.svg"),
                is_equippable_by_all: false,
            }];
            let bad_part_list2 = vec![Part {
                part_type: PartType::Fixed,
                z: 0,
                equippable: vec![],
                part_uri: String::from("ipfs://backgrounds/2.svg"),
                is_equippable_by_all: true,
            }];

            let mut rmrk = init();

            // verify add/get parts
            assert!(rmrk.get_parts_count() == 0);
            assert!(rmrk.add_part_list(part_list.clone()).is_ok());
            assert_eq!(rmrk.get_parts_count(), part_list.len() as u32);
            assert_eq!(rmrk.get_part(0).unwrap().z, part_list[0].z);
            assert_eq!(rmrk.get_part(0).unwrap().part_uri, part_list[0].part_uri);

            // verify array of equippable addresses
            assert!(rmrk
                .ensure_equippable(PART_ID0, EQUIPABLE_ADDRESS1.into())
                .is_ok());
            assert!(rmrk
                .ensure_equippable(PART_ID0, EQUIPABLE_ADDRESS2.into())
                .is_ok());
            assert!(!rmrk
                .ensure_equippable(PART_ID1, EQUIPABLE_ADDRESS2.into())
                .is_ok());

            assert!(!rmrk.is_equippable_by_all(PART_ID0));
            assert!(rmrk.set_equippable_by_all(PART_ID0).is_ok());
            assert!(rmrk.is_equippable_by_all(PART_ID0));
            assert!(!rmrk.is_equippable_by_all(42));

            assert!(rmrk.reset_equippable_addresses(PART_ID0).is_ok());
            assert!(!rmrk.is_equippable_by_all(PART_ID0));
            assert!(!rmrk
                .ensure_equippable(PART_ID0, EQUIPABLE_ADDRESS1.into())
                .is_ok());
            assert!(rmrk
                .add_equippable_addresses(
                    PART_ID0,
                    vec![EQUIPABLE_ADDRESS1.into(), EQUIPABLE_ADDRESS2.into()]
                )
                .is_ok());
            assert!(rmrk
                .ensure_equippable(PART_ID0, EQUIPABLE_ADDRESS1.into())
                .is_ok());
            assert_eq!(
                rmrk.add_equippable_addresses(PART_ID1, vec![EQUIPABLE_ADDRESS1.into()]),
                Err(RmrkError::PartIsNotSlot.into())
            );
            assert_eq!(
                rmrk.reset_equippable_addresses(PART_ID1),
                Err(RmrkError::PartIsNotSlot.into())
            );
            assert_eq!(
                rmrk.set_equippable_by_all(PART_ID1),
                Err(RmrkError::PartIsNotSlot.into())
            );
            assert_eq!(
                rmrk.add_part_list(bad_part_list1.clone()),
                Err(RmrkError::BadConfig.into())
            );
            assert_eq!(
                rmrk.add_part_list(bad_part_list2.clone()),
                Err(RmrkError::BadConfig.into())
            );

            assert!(!rmrk
                .ensure_equippable(PART_ID0, EQUIPABLE_ADDRESS3.into())
                .is_ok());

            // verify set/get base metadata
            assert_eq!(rmrk.get_base_metadata(), "");
            assert!(rmrk
                .setup_base(String::from("ipfs://base_metadata"))
                .is_ok());
            assert_eq!(rmrk.get_base_metadata(), "ipfs://base_metadata");

            // assert_eq!(1, ink_env::test::recorded_events().count());
        }

        #[ink::test]
        fn equip_works() {
            const ASSET_URI: &str = "asset_uri/";
            const ASSET_ID: AssetId = 1;
            const TOKEN_ID1: Id = Id::U64(1);
            const TOKEN_ID2: Id = Id::U64(2);
            const NOT_EQUIPABLE_ADDRESS: [u8; 32] = [1; 32];
            const CHILD_COLLECTION_ADDRESS: [u8; 32] = [10; 32];
            const CHILD_TOKEN_ID: Id = Id::U64(2);
            const CHILD_ASSET_ID: AssetId = 2;
            const EQUIPPABLE_GROUP_ID: EquippableGroupId = 1;

            const PART_ID0: PartId = 0;
            const PART_ID1: PartId = 1;
            let part_list = vec![
                // Background option 1
                Part {
                    part_type: PartType::Slot,
                    z: 0,
                    equippable: vec![CHILD_COLLECTION_ADDRESS.into()],
                    part_uri: String::from("ipfs://backgrounds/1.svg"),
                    is_equippable_by_all: false,
                },
                // Background option 2
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    part_uri: String::from("ipfs://backgrounds/2.svg"),
                    is_equippable_by_all: false,
                },
            ];

            let accounts = default_accounts();
            let mut kanaria = init();

            // Base setup
            assert!(kanaria.add_part_list(part_list.clone()).is_ok());

            // Add asset to kanaria collection
            assert!(kanaria
                .add_asset_entry(
                    ASSET_ID,
                    EQUIPPABLE_GROUP_ID,
                    String::from(ASSET_URI),
                    vec![PART_ID0]
                )
                .is_ok());
            assert_eq!(1, ink_env::test::recorded_events().count());

            // equip fails, token does not exist. Not minted yet
            assert_eq!(
                kanaria.equip(
                    TOKEN_ID1,
                    ASSET_ID,
                    PART_ID0,
                    (CHILD_COLLECTION_ADDRESS.into(), CHILD_TOKEN_ID),
                    CHILD_ASSET_ID,
                ),
                Err(PSP34Error::TokenNotExists.into())
            );

            // Bob mints kanaria
            // set_sender(accounts.bob);
            assert!(kanaria.mint(accounts.bob).is_ok());
            assert_eq!(2, ink_env::test::recorded_events().count());

            // equip fails, caller not token owner
            // set_sender(accounts.alice);
            assert_eq!(
                kanaria.equip(
                    TOKEN_ID1,
                    ASSET_ID,
                    PART_ID0,
                    (CHILD_COLLECTION_ADDRESS.into(), CHILD_TOKEN_ID),
                    CHILD_ASSET_ID,
                ),
                Err(RmrkError::NotTokenOwner.into())
            );

            // add asset to kanaria token
            assert!(kanaria
                .add_asset_to_token(TOKEN_ID1, ASSET_ID, None)
                .is_ok());
            // assert_eq!(4, ink_env::test::recorded_events().count());

            let asset = kanaria
                .get_asset_and_equippable_data(TOKEN_ID1, ASSET_ID)
                .unwrap();
            assert!(asset.equippable_group_id == EQUIPPABLE_GROUP_ID);
            assert!(asset.asset_uri.len() > 0);
            assert!(asset.part_ids[0] == PART_ID0);

            // equip fails, AddressNotEquippable
            set_sender(accounts.bob);
            assert_eq!(
                kanaria.equip(
                    TOKEN_ID1,
                    ASSET_ID,
                    PART_ID0,
                    (NOT_EQUIPABLE_ADDRESS.into(), CHILD_TOKEN_ID),
                    CHILD_ASSET_ID,
                ),
                Err(RmrkError::AddressNotEquippable.into())
            );

            // equip works
            assert_eq!(3, ink_env::test::recorded_events().count());
            assert!(kanaria
                .equip(
                    TOKEN_ID1,
                    ASSET_ID,
                    PART_ID0,
                    (CHILD_COLLECTION_ADDRESS.into(), CHILD_TOKEN_ID),
                    CHILD_ASSET_ID,
                )
                .is_ok());

            assert_eq!(
                kanaria.get_equipment(TOKEN_ID1, PART_ID0),
                Some(Equipment {
                    asset_id: ASSET_ID,
                    child_asset_id: CHILD_ASSET_ID,
                    child_nft: (CHILD_COLLECTION_ADDRESS.into(), CHILD_TOKEN_ID)
                })
            );

            // check AssetEquipped event is emitted
            // assert_eq!(4, ink_env::test::recorded_events().count());

            // equip fails, TargetAssetCannotReceiveSlot
            assert_eq!(
                kanaria.equip(
                    TOKEN_ID1,
                    ASSET_ID,
                    PART_ID1,
                    (CHILD_COLLECTION_ADDRESS.into(), CHILD_TOKEN_ID),
                    CHILD_ASSET_ID,
                ),
                Err(RmrkError::TargetAssetCannotReceiveSlot.into())
            );

            // equip fails, SlotAlreayUsed
            assert_eq!(
                kanaria.equip(
                    TOKEN_ID1,
                    ASSET_ID,
                    PART_ID0,
                    (CHILD_COLLECTION_ADDRESS.into(), CHILD_TOKEN_ID),
                    CHILD_ASSET_ID,
                ),
                Err(RmrkError::SlotAlreayUsed.into())
            );

            // un-equip token
            assert!(kanaria.unequip(TOKEN_ID1, PART_ID0).is_ok());
            assert_eq!(kanaria.get_equipment(TOKEN_ID1, PART_ID0), None);

            // check AssetEquipped event is emitted
            // assert_eq!(6, ink_env::test::recorded_events().count());
        }

        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
    }
}
