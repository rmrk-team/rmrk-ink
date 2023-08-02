use rmrk_storage::RmrkStorageSelector;
pub type Result<T> = std::result::Result<T, ()>;

pub trait Internal {
    fn _check_value(&mut self, transfered_value: u128, mint_amount: u64) -> Result<()>;
}

impl<T> Internal for T
where
    T: RmrkStorageSelector,
{
    fn _check_value(&mut self, transfered_value: u128, mint_amount: u64) -> Result<()> {
        if let Some(value) =
            (mint_amount as u128).checked_mul(self.storage().minting.price_per_mint)
        {
            if transfered_value == value {
                return Ok(())
            }
        }

        Err(())
    }
}
