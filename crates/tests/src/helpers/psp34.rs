use rmrk_example_equippable::rmrk_example_equippable::RmrkRef;

use ink::{
    env::DefaultEnvironment,
    primitives::AccountId,
};
use ink_e2e::{
    build_message,
    AccountKeyring,
    Client,
    PairSigner,
};

use openbrush::contracts::psp34::{
    extensions::enumerable::*,
    psp34_external::PSP34,
};

pub async fn call_transfer(
    client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    contract: AccountId,
    caller: AccountKeyring,
    to: AccountId,
    id: Id,
) {
    let signer = PairSigner::new(caller.pair());
    let approve = build_message::<RmrkRef>(contract.clone())
        .call(|c| PSP34::transfer(c, to, id.clone(), Default::default()));
    let _ = client
        .call(&signer, approve, 0, None)
        .await
        .expect("Call transfer failed");
}

pub async fn call_approve(
    client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    contract: AccountId,
    caller: AccountKeyring,
    operator: AccountId,
    id: Id,
) {
    let signer = PairSigner::new(caller.pair());
    let approve = build_message::<RmrkRef>(contract.clone())
        .call(|c| PSP34::approve(c, operator, Some(id.clone()), true));
    let _ = client
        .call(&signer, approve, 0, None)
        .await
        .expect("Call approve failed");
}

pub async fn query_owner_of(
    client: &mut Client<ink_e2e::PolkadotConfig, DefaultEnvironment>,
    contract: AccountId,
    caller: AccountKeyring,
    id: Id,
) -> AccountId {
    let signer = PairSigner::new(caller.pair());
    let get = build_message::<RmrkRef>(contract.clone()).call(|c| PSP34::owner_of(c, id.clone()));
    client
        .call_dry_run(&signer, &get, 0, None)
        .await
        .return_value()
        .expect("Query owner of failedf")
}
