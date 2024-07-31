use crate::tool::assets;

use solana_program::{
    msg,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};


pub fn transfer(_program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {

    // account
    if accounts.len() < 2 {
        msg!("payer, to");
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let accounts_iter = &mut accounts.iter();
    let from: &AccountInfo = next_account_info(accounts_iter)?;
    let to: &AccountInfo = next_account_info(accounts_iter)?;

    assets::transfer_on_site(from, to, lamports)?;

    Ok(())
}

pub fn transfer_with_cpi(_program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {

    // account
    if accounts.len() < 3 {
        msg!("payer, to, system");
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let accounts_iter = &mut accounts.iter();
    let payer: &AccountInfo = next_account_info(accounts_iter)?;
    let to: &AccountInfo = next_account_info(accounts_iter)?;
    let system: &AccountInfo = next_account_info(accounts_iter)?;

    assets::transfer_with_cpi(payer, to, system, lamports)?;

    Ok(())
}