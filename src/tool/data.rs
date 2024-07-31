// 处理 string and u32.
use solana_program::hash;

/// 合并两个字节流
pub fn combine(buffer1: &[u8], buffer2: Vec<u8>) -> Vec<u8> {
    let mut combined = Vec::with_capacity(buffer1.len() + buffer2.len());
    combined.extend_from_slice(buffer1);
    combined.extend(buffer2);
    combined
}

/// 字节流截取前面几个字节
pub fn cut_prefix(orign: &[u8], prefix: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(prefix);
    buffer.copy_from_slice(&orign[..prefix]);
    buffer
}


// 参考solidity，前四个字节作为函数选择器，参数的数据将从第五个字节开始，每个参数都被填充为个32字节。
fn method_selector(method: &str) -> [u8; 4] {
    let hash = hash::hash(method.as_bytes());
    let mut selector_hash: [u8; 4] = [0, 0, 0, 0];
    selector_hash.copy_from_slice(&hash.as_ref()[..4]);
    return selector_hash;
}