use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    Helloworld,
    Greet,                       //程序内部处理GreetingInfo，不会通过指令传递
    CreateAccount(AddressInfo),  //通过指令传递AddressInfo，并且存储到程序中
    CreatePDA(Vec<u8>),
    CloseAccount,
    Transfer(u64),
    TransferCPI(u64),
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}