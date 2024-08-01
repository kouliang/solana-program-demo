use rust_client::*;
use solana_sdk::signer::Signer;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

use kl::Instruction;

fn main() {
    let wallet = Wallet::default(RpcType::Localhost);
    let program_id = wallet::keypair_frome_file(&std::env::current_dir().unwrap().join("target/deploy/kl-keypair.json")).pubkey();
    
    // from account owner must be program_id
    let from = wallet::pubkey_from_str("BdzfzYqiSL5sAarVnUxnYwZrruASq5gziizVVzFncVfp");

    //receiver
    let receiver = wallet::keypair_new();
    println!("receiver: {:?}", receiver.pubkey());

    transfer(&wallet, &program_id, &from, &receiver.pubkey(), 10000000); //from => to


    println!("============================");

    transfer_with_cpi(&wallet, &program_id, &receiver.pubkey(), 10000000); //wallet.payer => to
}

/// from account owner must be program_id
fn transfer(wallet: &Wallet, program_id: &Pubkey, from: &Pubkey, to: &Pubkey, lamports: u64) {

    // instruction
    let account_metas = vec![
        AccountMeta::new(*from, false),
        AccountMeta::new(*to, false),
    ];
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::Transfer(lamports)).unwrap();
    let instruction1 = instruction_builder::origin(*program_id, &instruction_data, account_metas);
    
    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    wallet.send_transaction(&transaction);
}

fn transfer_with_cpi(wallet: &Wallet, program_id: &Pubkey, to: &Pubkey, lamports: u64) {

    // instruction
    let account_metas = vec![
        AccountMeta::new(wallet.pubkey(), true),
        AccountMeta::new(*to, false),
        AccountMeta::new(solana_program::system_program::id(), false),
    ];
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::TransferCPI(lamports)).unwrap();
    let instruction1 = instruction_builder::origin(*program_id, &instruction_data, account_metas);
    
    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    wallet.send_transaction(&transaction);
}