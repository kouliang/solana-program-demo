use rust_client::*;
use solana_sdk::signer::Signer;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

use kl::Instruction;

fn main() {
    let wallet = Wallet::default(RpcType::Localhost);
    let program_id = wallet::keypair_frome_file(&std::env::current_dir().unwrap().join("target/deploy/kl-keypair.json")).pubkey();
    
    create_pda(&wallet, &program_id, b"hello");
}

fn create_pda(wallet: &Wallet, program_id: &Pubkey, seed: &[u8]) -> Pubkey {
    let (pda, bump_seed) = Pubkey::find_program_address(&[seed], &program_id);
    println!("pda: {}", pda.to_string());
    println!("bump: {}", bump_seed);

    // instruction
    let account_metas = vec![
        AccountMeta::new(wallet.payer.pubkey(), true),
        AccountMeta::new(pda, false),
        AccountMeta::new(solana_program::system_program::id(), false),
    ];
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::CreatePDA(seed.into())).unwrap();
    let instruction1 = instruction_builder::origin(*program_id, &instruction_data, account_metas);
    
    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    wallet.send_transaction(&transaction);
    println!("new pda account: {}", pda);

    pda
}