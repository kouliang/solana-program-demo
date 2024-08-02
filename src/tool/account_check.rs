use solana_program::{
    msg,
    entrypoint::ProgramResult,
    program_error::ProgramError,

    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},

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

/**
 *  只有 lamports 和 data 是可以修改的。两者都使用了 RefCell 结构，实现内部可变性。
 *  pub struct AccountInfo<'a> {
        pub key: &'a Pubkey,
        pub lamports: Rc<RefCell<&'a mut u64>>,
        pub data: Rc<RefCell<&'a mut [u8]>>,
        pub owner: &'a Pubkey,
        pub rent_epoch: Epoch,
        pub is_signer: bool,
        pub is_writable: bool,
        pub executable: bool,
    }
 */
fn check_accounts(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {

    // 检查 数量
    // =====================================
    if accounts.len() < 3 {
        msg!("payer, account_to_create, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // accounts是有序的
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let account2 = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Checking if payer account is the signer
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Checking if the system program is valid
    if system_program.key.ne(&id_system()) {
        return Err(ProgramError::IncorrectProgramId);
    }
    if system_program.key != &id_system() {
        return Err(ProgramError::IncorrectProgramId);
    };


    // 检查 余额
    if account2.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    };

    // Checking if hello state account is rent exempt
    if !Rent::get()?.is_exempt(account2.lamports(), 1) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Checking if hello state account is writable
    if !account2.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Checking if account's owner is the current program
    if account2.owner.ne(&program_id) {
        return Err(ProgramError::IllegalOwner);
    }
    
    Ok(())
}


