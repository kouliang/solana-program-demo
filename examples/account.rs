use rust_client::*;
use solana_sdk::signer::Signer;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

use kl::Instruction;
use kl::AddressInfo;
use kl::BorshDeserialize;

fn main() {
    let wallet = Wallet::default(RpcType::Localhost);
    let program_id = wallet::keypair_frome_file(&std::env::current_dir().unwrap().join("target/deploy/kl-keypair.json")).pubkey();
    
    let new_account_id = create(&wallet, &program_id);
    println!("============================");
    close(&wallet, &program_id, new_account_id);
}

fn create(wallet: &Wallet, program_id: &Pubkey) -> Pubkey {

    // new keypair
    let new_keypair = wallet::keypair_new();

    // instruction
    let account_metas = vec![
        AccountMeta::new(wallet.payer.pubkey(), true),
        AccountMeta::new(new_keypair.pubkey(), true),
        AccountMeta::new(solana_program::system_program::id(), false),
    ];
    let address_info = AddressInfo {name: "xx".to_string(), house_number: 9, street: "liwan".to_string(), city: "guangzhou".to_string()};
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::CreateAccount(address_info)).unwrap();
    let instruction1 = instruction_builder::origin(*program_id, &instruction_data, account_metas);
    
    let signing_keypairs = &[&wallet.payer, &new_keypair];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    wallet.send_transaction(&transaction);

    // show data
    let new_account_id = new_keypair.pubkey();
    println!("new account: {}", new_account_id);

    let account = wallet.client.get_account(&new_account_id).unwrap();
    let info = AddressInfo::try_from_slice(&account.data).unwrap();
    println!("{:?}", info);

    new_account_id
}

fn close(wallet: &Wallet, program_id: &Pubkey, target: Pubkey) {
    // instruction
    let account_metas = vec![
        AccountMeta::new(wallet.payer.pubkey(), true),
        AccountMeta::new(target, false),
        AccountMeta::new(solana_program::system_program::id(), false),
    ];
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::CloseAccount).unwrap();
    let instruction1 = instruction_builder::origin(*program_id, &instruction_data, account_metas);
    
    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    wallet.send_transaction(&transaction);
}