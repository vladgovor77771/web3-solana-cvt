mod wallet;
mod certora_verification_harness;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::wallet::Wallet;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if accounts.is_empty() || instruction_data.is_empty() {
        return Err(ProgramError::InvalidAccountData);
    }

    let caller_account = next_account_info(&mut accounts.iter())?;
    if !caller_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut wallet = Wallet::unpack_unchecked(&caller_account.try_borrow_data()?)?;

    // Parse the instruction_data
    // Assuming the first byte is the method (0 for add_money, 1 for spend_money)
    // and the rest of the bytes represent the amount (as u64)
    if instruction_data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let method = instruction_data[0];
    let amount = u64::from_le_bytes(
        instruction_data[1..9]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    match method {
        0 => wallet.add_money(amount)?,
        1 => wallet.spend_money(amount)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Wallet::pack(wallet, &mut caller_account.try_borrow_mut_data()?)?;

    Ok(())
}
