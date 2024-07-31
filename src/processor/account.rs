// 创建新账户，并且在账户中存储数据

use crate::state::AddressInfo;
use crate::tool::account_create;
use crate::tool::account_check;
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    msg,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn create(program_id: &Pubkey, accounts: &[AccountInfo], info: AddressInfo) -> ProgramResult {

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
    let space = borsh::to_vec(&info).unwrap().len();
    account_create::create(payer, new, system, space, program_id)?;
    msg!("created account: {}", new.key);

    // fill data
    // info.serialize(&mut *new.data.borrow_mut())?;
    info.serialize(&mut &mut new.data.borrow_mut()[..])?;

    Ok(())
}

pub fn close(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    if accounts.len() < 3 {
        msg!("payer, target, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let accounts_iter = &mut accounts.iter();
    let payer: &AccountInfo = next_account_info(accounts_iter)?;
    let target: &AccountInfo = next_account_info(accounts_iter)?;
    let system: &AccountInfo = next_account_info(accounts_iter)?;

    account_create::close(payer, target, system)?;
    msg!("{} close success", target.key);

    Ok(())
}