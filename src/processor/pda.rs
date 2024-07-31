use crate::tool::account_create;
use crate::tool::account_check;

use solana_program::{
    msg,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};


pub fn create(program_id: &Pubkey, accounts: &[AccountInfo], seed: &[u8]) -> ProgramResult {

    // account
    if accounts.len() < 3 {
        msg!("payer, account_to_create, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let accounts_iter = &mut accounts.iter();
    let payer: &AccountInfo = next_account_info(accounts_iter)?;
    let new: &AccountInfo = next_account_info(accounts_iter)?;
    let system: &AccountInfo = next_account_info(accounts_iter)?;
    assert!(account_check::id_is_system(system.key), "id_is_system check failed");

    // space
    let space: usize = 10;

    account_create::create_pda(payer, new, system, space, seed, program_id)?;
    msg!("created pad: {}", new.key);

    Ok(())
}