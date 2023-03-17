#[cfg(all(test, feature = "e2e-tests"))]
pub mod e2e_tests {

    use common::item::{
        Item,
        Items,
    };
    use factory_trade::contract::FactoryRef;
    use ink::{
        env::DefaultEnvironment,
        primitives::AccountId,
    };
    use ink_e2e::{
        build_message,
        log_info,
        Client,
        PairSigner,
    };

    use collection_item::contract::ItemRef;
    use ink_e2e::{
        AccountKeyring,
        AccountKeyring::{
            Alice,
            Bob,
            Eve,
        },
    };
    use openbrush::{
        contracts::psp34::{
            extensions::enumerable::*,
            psp34_external::PSP34,
        },
        traits,
    };

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub async fn call_mint_to(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        to: AccountKeyring,
    ) {
        let to = ink_e2e::account_id(to);
        let mint = build_message::<ItemRef>(acc_id.clone()).call(|c| c.mint_item(to));
        let mint_res = client
            .call(&ink_e2e::alice(), mint, 0, None)
            .await
            .expect("mint failed");
    }

    pub async fn call_mint_many_to(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        to: AccountKeyring,
        amount: u32,
    ) {
        for _ in 0..amount {
            call_mint_to(client, acc_id, to).await;
        }
    }

    pub async fn call_approve(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        caller: AccountKeyring,
        operator: AccountId,
        id: Id,
    ) {
        let pair = PairSigner::new(caller.pair());
        let approve = build_message::<ItemRef>(acc_id.clone())
            .call(|c| PSP34::approve(c, operator, Some(id.clone()), true));

        let approve_res = client
            .call(&pair, approve, 0, None)
            .await
            .expect("approve failed");
    }

    pub async fn call_approve_many(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        caller: AccountKeyring,
        operator: AccountId,
        ids: &Vec<(AccountId, Id)>,
    ) {
        for (acc_id, id) in ids {
            call_approve(client, acc_id.clone(), caller, operator, id.clone()).await
        }
    }

    pub async fn call_find_trade(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        caller: AccountKeyring,
        partner: AccountKeyring,
    ) -> AccountId {
        let pair = PairSigner::new(caller.pair());
        let partner = ink_e2e::account_id(partner);

        let get = build_message::<FactoryRef>(acc_id.clone()).call(|c| c.find_trade(partner));

        let res = client.call_dry_run(&pair, &get, 0, None).await;
        let logs = res.debug_message();
        log_info(&logs);
        res.return_value().expect("No trade")
    }

    pub async fn call_owner_of(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        id: Id,
    ) -> AccountId {
        let get = build_message::<ItemRef>(acc_id.clone()).call(|c| PSP34::owner_of(c, id.clone()));
        client
            .call_dry_run(&ink_e2e::alice(), &get, 0, None)
            .await
            .return_value()
            .expect("No owner")
    }

    pub async fn assert_owner_of(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        id: Id,
        expected_owner: AccountId,
    ) {
        let owner = call_owner_of(client, acc_id, id).await;
        assert_eq!(owner, expected_owner);
    }

    pub async fn assert_owner_of_many(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        ids: &Vec<(AccountId, Id)>,
        expected_owner: AccountId,
    ) {
        for (acc_id, id) in ids {
            assert_owner_of(client, acc_id.clone(), id.clone(), expected_owner);
        }
    }

    pub async fn init_factory(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    ) -> AccountId {
        let escrow_hash = client
            .upload("escrow", &ink_e2e::alice(), None)
            .await
            .expect("instantiate failed")
            .code_hash;

        let factory_constructor = FactoryRef::new(escrow_hash);
        client
            .instantiate(
                "factory_trade",
                &ink_e2e::alice(),
                factory_constructor,
                0,
                None,
            )
            .await
            .expect("instantiate failed")
            .account_id
    }

    pub async fn call_cancel(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        caller: AccountKeyring,
        partner: AccountKeyring,
    ) {
        let pair = PairSigner::new(caller.pair());
        let partner = ink_e2e::account_id(partner);
        let handle = build_message::<FactoryRef>(acc_id.clone()).call(|c| c.cancel_trade(partner));
        let res = client
            .call(&pair, handle, 0, None)
            .await
            .expect("cancel failed");
        let logs = res.debug_message();
        log_info(&logs);
    }

    pub async fn call_handle(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        caller: AccountKeyring,
        partner: AccountKeyring,
        offer: &Items,
        ask: &Items,
    ) {
        let pair = PairSigner::new(caller.pair());
        let partner = ink_e2e::account_id(partner);
        let handle = build_message::<FactoryRef>(acc_id.clone())
            .call(|c| c.handle(partner, offer.clone(), ask.clone()));
        let res = client
            .call(&pair, handle, 0, None)
            .await
            .expect("handle failed");
        let logs = res.debug_message();
        log_info(&logs);
    }

    pub async fn strap_bob_eve_with_items(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    ) -> (AccountId, Items, Items) {
        let factory_id = init_factory(client).await;
        let cid_a = init_item(client, "item_a").await;
        let cid_b = init_item(client, "item_b").await;
        let cid_c = init_item(client, "item_c").await;

        call_mint_many_to(client, cid_a, Bob, 3).await; // 1 - 3
        call_mint_many_to(client, cid_a, Eve, 3).await; // 4 - 6

        call_mint_many_to(client, cid_b, Bob, 3).await; // 1 - 3
        call_mint_many_to(client, cid_b, Eve, 3).await; // 4 - 6

        call_mint_many_to(client, cid_c, Bob, 3).await; // 1 - 3
        call_mint_many_to(client, cid_c, Eve, 3).await; // 4 - 6

        let bob_items: Items = vec![
            Item::PSP34(cid_a.clone(), Id::U64(1)),
            Item::PSP34(cid_b.clone(), Id::U64(2)),
            Item::PSP34(cid_c.clone(), Id::U64(3)),
        ]
        .into();

        let eve_items: Items = vec![
            Item::PSP34(cid_a.clone(), Id::U64(4)),
            Item::PSP34(cid_b.clone(), Id::U64(5)),
            Item::PSP34(cid_c.clone(), Id::U64(6)),
        ]
        .into();

        (factory_id, bob_items, eve_items)
    }

    pub async fn strap_bob_eve_approvals(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        factory_id: AccountId,
        bob_items: &Items,
        eve_items: &Items,
    ) -> (Vec<(AccountId, Id)>, Vec<(AccountId, Id)>) {
        let bob_psp34 = bob_items.filter_psp34();
        let eve_psp34 = eve_items.filter_psp34();
        call_approve_many(client, Bob, factory_id, &bob_psp34).await;
        call_approve_many(client, Eve, factory_id, &eve_psp34).await;
        assert_owner_of_many(client, &bob_psp34, ink_e2e::account_id(Bob)).await;
        assert_owner_of_many(client, &eve_psp34, ink_e2e::account_id(Eve)).await;
        (bob_psp34, eve_psp34)
    }

    pub async fn assert_bob_eve_owns(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        items_a: &Vec<(AccountId, Id)>,
        items_b: &Vec<(AccountId, Id)>,
    ) {
        assert_owner_of_many(client, &items_a, ink_e2e::account_id(Bob)).await;
        assert_owner_of_many(client, &items_b, ink_e2e::account_id(Eve)).await;
    }

    pub async fn assert_escrow_owns(
        mut client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        factory_id: AccountId,
        items: &Vec<(AccountId, Id)>,
    ) {
        let escrow_id = call_find_trade(client, factory_id, Bob, Eve).await;
        assert_owner_of_many(client, items, escrow_id).await;
    }
}
