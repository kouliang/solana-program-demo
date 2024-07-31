use rust_client::*;
use solana_sdk::signer::Signer;

use kl::Instruction;

fn main() {
    let wallet = Wallet::default(RpcType::Localhost);
    let program_id = wallet::keypair_frome_file(&std::env::current_dir().unwrap().join("target/deploy/kl-keypair.json")).pubkey();

    let instruction_data: Vec<u8> = borsh::to_vec(&Instruction::Helloworld).unwrap();
    let instruction1 = instruction_builder::origin(program_id, &instruction_data, vec![]);
    
    let signing_keypairs = &[&wallet.payer];
    let transaction = transaction_builder::signed_independent(&wallet, &[instruction1], signing_keypairs);

    rust_client::send_transaction(&wallet, &transaction);
}