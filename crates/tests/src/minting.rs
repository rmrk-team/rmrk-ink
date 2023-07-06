use crate::helpers::{
    alice,
    bob,
    eve,
    psp34::{
        call_approve,
        call_transfer,
        query_owner_of,
    },
    rmrk::{
        call_mint,
        init_contract,
    },
};

use openbrush::contracts::psp34::extensions::enumerable::*;

use ink_e2e::AccountKeyring::{
    Alice,
    Bob,
    Eve,
};

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn can_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let contract = init_contract(&mut client, Alice, "", "", "", 1000, "").await;
    call_mint(&mut client, contract, Alice, Bob).await;
    let owner = query_owner_of(&mut client, contract, Alice, Id::U64(1)).await;
    assert!(owner == bob());
    Ok(())
}

#[ink_e2e::test]
async fn can_approve_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let contract = init_contract(&mut client, Alice, "", "", "", 1000, "").await;
    call_mint(&mut client, contract, Alice, Bob).await;
    call_approve(&mut client, contract, Bob, eve(), Id::U64(1)).await;
    call_transfer(&mut client, contract, Eve, alice(), Id::U64(1)).await;
    let owner = query_owner_of(&mut client, contract, Alice, Id::U64(1)).await;
    assert!(owner == alice());
    Ok(())
}
