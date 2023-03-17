#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {

    use ink_e2e::{
        log_error,
        log_info,
        AccountKeyring,
        AccountKeyring::{
            Alice,
            Bob,
            Eve,
        },
        PairSigner,
    };

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    use crate::util::e2e_tests::*;
    use openbrush::{
        contracts::psp34::{
            extensions::enumerable::*,
            psp34_external::PSP34,
        },
        traits,
    };
    use rmrk_example_equippable::rmrk_example_equippable::RmrkRef;

    async fn init_contract(
        client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
        name: &str,
        symbol: &str,
        max_supply: u64,
    ) -> AccountId {
        let item_a_constructor = RmrkRef::new(
            traits::String::from(name),
            traits::String::from(symbol),
            traits::String::from("ipfs//"),
            max_supply,
            traits::String::from("ipfs//"),
        );
        client
            .instantiate(
                "rmrk_example_equippable",
                &ink_e2e::alice(),
                item_a_constructor,
                0,
                None,
            )
            .await
            .expect("instantiate failed")
            .account_id
    }

    #[ink_e2e::test]
    async fn can_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let contract = init_contract(&mut client, "", "", 0);
        Ok(())
    }
}
