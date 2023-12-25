use crate::{wallet::Wallet,process_instruction};
use cvt;
use solana_program::{account_info::AccountInfo, program_pack::Pack};

#[no_mangle]
pub fn check_add_new_account() {
    let program_id = cvt::CVT_nondet_pubkey();
    let acc_infos = [cvt::nondet::<AccountInfo>()];
    let amount = cvt::nondet::<u64>();
    cvt::CVT_assume(amount > 0);

    let mut instruction = Vec::new();
    instruction.push(0);
    instruction.extend_from_slice(&amount.to_le_bytes());

    process_instruction(&program_id, &acc_infos, instruction.as_slice()).unwrap();

    let wallet_after = Wallet::unpack_unchecked(&acc_infos[0].try_borrow_data().unwrap()).unwrap();

    cvt::CVT_assert(wallet_after.balance == amount);
}

// pub fn check_existing_overflow() {

// }
