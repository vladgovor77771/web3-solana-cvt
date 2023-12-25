use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

pub struct Wallet {
    pub balance: u64,
}

impl Sealed for Wallet {}

impl Pack for Wallet {
    const LEN: usize = 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.is_empty() {
            return Ok(Wallet { balance: 0 });
        }
        if src.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let balance = u64::from_le_bytes(src.try_into().expect("Invalid balance data"));

        Ok(Wallet { balance })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.balance.to_le_bytes());
    }
}

impl Wallet {
    pub fn add_money(&mut self, amount: u64) -> Result<(), ProgramError> {
        self.balance += amount;
        Ok(())
    }

    pub fn spend_money(&mut self, amount: u64) -> Result<(), ProgramError> {
        if self.balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        // undeflow impossible here due to logic check above
        self.balance -= amount;
        Ok(())
    }
}
