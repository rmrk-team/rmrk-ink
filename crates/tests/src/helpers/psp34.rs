#[cfg(all(test, feature = "e2e-tests"))]
pub mod e2e_tests {

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

    pub async fn call_approve<C>(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        caller: AccountKeyring,
        operator: AccountId,
        id: Id,
    ) {
        let pair = PairSigner::new(caller.pair());
        let approve = build_message::<C>(acc_id.clone())
            .call(|c| PSP34::approve(c, operator, Some(id.clone()), true));

        let approve_res = client
            .call(&pair, approve, 0, None)
            .await
            .expect("approve failed");
    }

    pub async fn call_owner_of<C>(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        acc_id: AccountId,
        id: Id,
    ) -> AccountId {
        let get = build_message::<C>(acc_id.clone()).call(|c| PSP34::owner_of(c, id.clone()));
        client
            .call_dry_run(&ink_e2e::alice(), &get, 0, None)
            .await
            .return_value()
            .expect("No owner")
    }
}
