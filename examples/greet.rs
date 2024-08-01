use rust_client::*;
use solana_sdk::signer::Signer;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

use kl::Instruction;
use kl::GreetingInfo;
use kl::{BorshDeserialize, BorshSerialize};

fn show_data(wallet: &Wallet, address: Pubkey) {
    let account = wallet.client.get_account(&address).unwrap();
    let info = GreetingInfo::try_from_slice(&account.data).unwrap();

    println!("{:?}", info);
}

fn main() {
    let wallet = Wallet::default(RpcType::Localhost);
    let program_id = wallet::keypair_frome_file(&std::env::current_dir().unwrap().join("target/deploy/kl-keypair.json")).pubkey();
    
    let greeting1 = createaccount_and_invokeprogram(&wallet, program_id);
    let greeting2 = createaccount_and_invokeprogram(&wallet, program_id);

    invoke_program(&wallet, program_id, &[greeting1, greeting2]);

    show_data(&wallet, greeting1);
    show_data(&wallet, greeting2);
}

fn createaccount_and_invokeprogram(wallet: &Wallet, program_id: Pubkey) -> Pubkey {
    // data
    let greeting_account = GreetingInfo{counter:1};
    let mut buffer: Vec<u8> = Vec::new();
    greeting_account.serialize(&mut buffer).unwrap();
    let buffer_len = buffer.len();

    // newkeypair
    let greeting_kp = wallet::keypair_new();
    let greeting_pub = greeting_kp.pubkey();

    // instruction1
    let instruction1 = instruction_builder::create_account(wallet, &greeting_pub, &program_id, buffer_len);

    // instruction2
    let account_metas = vec![
        AccountMeta::new(greeting_pub, false),
    ];
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::Greet).unwrap();
    let instruction2 = instruction_builder::origin(program_id, &instruction_data, account_metas);

    // transaction
    let signing_keypairs = &[&wallet.payer, &greeting_kp];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1, instruction2], signing_keypairs);

    // send
    wallet.send_transaction(&transaction);

    println!("greeting_address: {}", greeting_pub);
    greeting_pub
}

fn invoke_program(wallet: &Wallet, program_id: Pubkey, greeting_accounts: &[Pubkey]) {

    let mut account_metas: Vec<AccountMeta> = Vec::new();
    for account in greeting_accounts {
        account_metas.push(AccountMeta::new(*account, false));
    }
    
    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::Greet).unwrap();
    let instruction1 = instruction_builder::origin(program_id, &instruction_data, account_metas);

    // transaction
    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    // send
    wallet.send_transaction(&transaction);
}