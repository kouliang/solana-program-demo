use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program_error::ProgramError,
};

/// from_account must owned by the program_id (like pda)  
/// to_account can by any account, if not exist, it will created automatically(System owned)  
pub fn transfer_on_site (
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    lamports: u64,
) -> ProgramResult {
    // Does the from account have enough lamports to transfer?
    if **from_account.try_borrow_lamports()? < lamports {
        return Err(ProgramError::InsufficientFunds);
    }
    // Debit from_account and credit to_account
    // **from_account.lamports.borrow_mut() -= amount_of_lamports;
    **from_account.try_borrow_mut_lamports()? -= lamports;
    **to_account.try_borrow_mut_lamports()? += lamports;
    Ok(())
}

/// from_account must owned by the system program, must is_signer  
/// to_account can by any account, if not exist, it will created automatically(System owned)  
pub fn transfer_with_cpi<'a> (
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system: &AccountInfo<'a>,
    lamports: u64,
) -> ProgramResult {

    let transfer_instruction = system_instruction::transfer(
        from_account.key,
        to_account.key,
        lamports
    );

    invoke(&transfer_instruction, &[from_account.clone(), to_account.clone(), system.clone()])
}