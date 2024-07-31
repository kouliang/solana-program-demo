use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::{invoke, invoke_signed},
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program_error::ProgramError,
};

pub fn create<'a>(
    payer: &AccountInfo<'a>,
    target: &AccountInfo<'a>,
    system: &AccountInfo<'a>,
    space: usize,
    owner: &Pubkey
) -> ProgramResult {
    // instruction
    let lamports = (Rent::get()?).minimum_balance(space);
    let create_instruction = system_instruction::create_account(
        payer.key,
        target.key,
        lamports,
        space as u64,
        owner);

    // invoke
    invoke(&create_instruction, &[payer.clone(), target.clone(), system.clone()])
}

pub fn create_pda<'a>(
    payer: &AccountInfo<'a>,
    pda: &AccountInfo<'a>,
    system: &AccountInfo<'a>, 
    space: usize,
    seed: &[u8],
    program_id: &Pubkey,
) -> ProgramResult {

    let (pdakey, bump_seed) = Pubkey::find_program_address(&[seed], program_id);
    assert!(pda.key.eq(&pdakey));

    let lamports = (Rent::get()?).minimum_balance(space).max(1);

    let create_instruction = system_instruction::create_account(
        payer.key, 
        pda.key, 
        lamports, 
        space as u64, 
        program_id);

    invoke_signed(
        &create_instruction, 
        &[payer.clone(), pda.clone(), system.clone()],
        &[&[seed, &[bump_seed]]])
}


pub fn close<'a>(
    payer: &AccountInfo<'a>,
    target: &AccountInfo<'a>,
    system: &AccountInfo<'a>,
) -> ProgramResult {

    // Send the rent back to the payer
    **payer.lamports.borrow_mut() += target.lamports();
    **target.lamports.borrow_mut() = 0;

    // Realloc the account to zero
    target.realloc(0, true)?;

    // Assign the account to the System Program
    target.assign(system.key);

    Ok(())
}