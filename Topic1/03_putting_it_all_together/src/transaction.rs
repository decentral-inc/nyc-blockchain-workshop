const HASH_BYTE_SIZE: usize = 32;


pub type sha256 = [u8; HASH_BYTE_SIZE];


#[derive(Debug)]
pub struct Transaction {
    to: str,
    sender: str,
    nonce: u64,
     
}