pub mod psp34;
pub mod rmrk;

use ink::primitives::AccountId;

pub fn bob() -> AccountId {
    ink_e2e::account_id(ink_e2e::AccountKeyring::Bob)
}

pub fn alice() -> AccountId {
    ink_e2e::account_id(ink_e2e::AccountKeyring::Alice)
}

pub fn eve() -> AccountId {
    ink_e2e::account_id(ink_e2e::AccountKeyring::Eve)
}
