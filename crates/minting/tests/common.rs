use ink_env::{
    test,
    AccountId,
};

use openbrush::contracts::psp34::extensions::enumerable::*;

pub const MAX_SUPPLY: u64 = 10;

pub trait Accessor {
    fn _last_token_id(&self) -> u64;
    fn _owners_token_by_index(
        &self,
        account: AccountId,
        index: u128,
    ) -> core::result::Result<Id, PSP34Error>;
}

pub fn check_mint_single_outcome<T: Accessor + PSP34>(rmrk: T, account: AccountId, amount: u32) {
    assert_eq!(rmrk.total_supply(), amount as u128);
    assert_eq!(rmrk.owner_of(Id::U64(amount as u64)), Some(account));
    assert_eq!(rmrk.balance_of(account), amount);
    assert_eq!(
        <T as Accessor>::_owners_token_by_index(&rmrk, account, 0),
        Ok(Id::U64(amount as u64))
    );

    assert_eq!(<T as Accessor>::_last_token_id(&rmrk), amount as u64);
    assert_eq!(amount as usize, ink_env::test::recorded_events().count());
}

pub fn check_mint_many_outcome<T: Accessor + PSP34>(
    rmrk: T,
    account: AccountId,
    num_of_mints: u64,
) {
    assert_eq!(rmrk.total_supply(), num_of_mints as u128);
    assert_eq!(rmrk.balance_of(account), num_of_mints as u32);

    for i in 0..num_of_mints {
        assert_eq!(
            <T as Accessor>::_owners_token_by_index(&rmrk, account, i as u128),
            Ok(Id::U64((i + 1) as u64))
        );
    }
    assert_eq!(
        num_of_mints as usize,
        ink_env::test::recorded_events().count()
    );
    assert_eq!(
        <T as Accessor>::_owners_token_by_index(&rmrk, account, num_of_mints as u128),
        Err(PSP34Error::TokenNotExists)
    );
}

pub fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
    test::default_accounts::<ink_env::DefaultEnvironment>()
}

pub fn set_sender(sender: AccountId) {
    ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
}
