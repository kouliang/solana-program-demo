use solana_program::{
    entrypoint::ProgramResult,
    msg,
};

pub fn sayhello() -> ProgramResult {
    msg!("hello solana!");

    Ok(())
}