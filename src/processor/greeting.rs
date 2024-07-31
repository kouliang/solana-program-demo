// 将其他账户中的数据每次 +1

use crate::state::GreetingInfo;
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info:: AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn greet(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("greeting");

    for account in accounts {
        assert!(account.is_writable, "Counter account must be writable");

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut greeting_account = GreetingInfo::try_from_slice(&account.data.borrow())?;
        greeting_account.counter += 1;
        greeting_account.serialize(&mut *account.data.borrow_mut())?;
        msg!("{}: {} time(s)!", account.key, greeting_account.counter);
    }    
    
    Ok(())
}