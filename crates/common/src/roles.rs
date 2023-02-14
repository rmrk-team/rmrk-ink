use openbrush::contracts::access_control::*;

pub const ADMIN: RoleType = DEFAULT_ADMIN_ROLE;
pub const CONTRIBUTOR: RoleType = ink_lang::selector_id!("CONTRIBUTOR");
