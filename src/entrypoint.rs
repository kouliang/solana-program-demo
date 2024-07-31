use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    hash::hash,
};

use crate::Instruction;
use crate::Instruction::*;
use crate::processor::*;
use borsh::BorshDeserialize;

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,       //当前程序的id
    accounts: &[AccountInfo],  //需要访问（读或写）的全部账户组成的有序数组
    instruction_data: &[u8],   //u8 字节数组
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Helloworld => helloworld::sayhello(),
        Greet => greeting::greet(program_id, accounts),
        CreateAccount(info) => account::create(program_id, accounts, info),
        CreatePDA(seed) => pda::create(program_id, accounts, seed.as_slice()),
        CloseAccount => account::close(program_id, accounts),
        Transfer(lamports) => transfer::transfer(program_id, accounts, lamports),
        TransferCPI(lamports) => transfer::transfer_with_cpi(program_id, accounts, lamports),
    }
}