#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod rmrk_example_mintable_new {

    use rmrk::rmrk_storage::{
        impl_rmrk_storage,
        RmrkStorage,
    };

    use rmrk::rmrk_minting::Minting;

    #[ink(storage)]
    pub struct MyContract {
        rmrk: RmrkStorage,
    }

    impl_rmrk_storage!(MyContract);

    impl MyContract {
        #[ink(constructor)]
        pub fn new(psp34: AccountId) -> Self {
            let mut rmrk = RmrkStorage::new(psp34);
            rmrk.minting.max_supply = Some(1000);
            MyContract { rmrk }
        }

        #[ink(message)]
        pub fn mint_example(&mut self) -> Result<(), rmrk::rmrk_traits::psp34::PSP34Error> {
            let to = self.env().caller();
            self.mint(to)
        }
    }
}
