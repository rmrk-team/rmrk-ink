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
use rmrk::traits::{
    minting_external,
    minting_external::Minting,
};
use rmrk_example_equippable::rmrk_example_equippable::*;

use ink_e2e::AccountKeyring;
use openbrush::{
    contracts::psp34::{
        extensions::enumerable::*,
        psp34_external::PSP34,
    },
    traits::String,
};

pub async fn init_contract(
    client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    caller: AccountKeyring,
    name: &str,
    symbol: &str,
    base_uri: &str,
    max_supply: u64,
    collection_uri: &str,
) -> AccountId {
    let signer = PairSigner::new(caller.pair());
    let contract_constructor = RmrkRef::new(
        String::from(name),
        String::from(symbol),
        String::from(base_uri),
        max_supply,
        String::from(collection_uri),
    );
    client
        .instantiate(
            "rmrk_example_equippable",
            &signer,
            contract_constructor,
            0,
            None,
        )
        .await
        .expect("instantiate failed")
        .account_id
}

pub async fn call_mint(
    client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    contract: AccountId,
    caller: AccountKeyring,
    to: AccountKeyring,
) {
    let signer = PairSigner::new(caller.pair());
    let to = ink_e2e::account_id(to);
    let call = build_message::<RmrkRef>(contract).call(|c| c.mint(to.clone()));
    client.call(&signer, call, 0, None).await.expect("Call mint failed");
}
