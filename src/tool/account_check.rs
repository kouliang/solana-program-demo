use solana_program::{
    msg,
    entrypoint::ProgramResult,
    program_error::ProgramError,

    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,

    system_program,
};

pub fn id_system() -> Pubkey {
    system_program::id()
}

pub fn id_is_current_program(id: &Pubkey) -> bool {
    system_program::check_id(id)
}

pub fn id_is_system(id: &Pubkey) -> bool {
    *id == id_system()
}

pub fn account_is_initialized(account: &AccountInfo) -> bool {
    account.lamports() != 0
}

fn check_accounts(accounts: &[AccountInfo]) -> ProgramResult {

    // 检查 accounts
    // =====================================
    if accounts.len() < 3 {
        msg!("payer, account_to_create, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // accounts是有序的
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;


    // 检查 AccountInfo
    // =====================================
    if account_to_create.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    };

    if system_program.key != &id_system() {
        return Err(ProgramError::IncorrectProgramId);
    };
    
    Ok(())
}


