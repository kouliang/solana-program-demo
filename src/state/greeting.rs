use borsh::{BorshDeserialize, BorshSerialize};

/// # Examples
///
/// ```
/// // struct -> bytes
/// let info = GreetingInfo {counter: 9};
/// let mut buffer: Vec<u8> = Vec::new();
/// info.serialize(&mut buffer).unwrap();
/// let data: Vec<u8> = borsh::to_vec(&info).unwrap();
/// 
/// // bytes -> struct  参数为 &Vec<u8> 或者 &[u8] 类型都可以
/// let info = GreetingInfo::try_from_slice(&buffer).unwrap();
/// ```
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingInfo {
    /// number of greetings
    pub counter: u32,
}