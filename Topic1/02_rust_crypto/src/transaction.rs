extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use chrono::prelude::*;

const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];


#[derive(Debug)]
pub struct Transaction<'a> {
    timestamp: i64,
    to: &'a str,
    sender:  &'a str,
    amount: u64,
    signature:  &'a str,
    data: &'a str,
}


impl <'a>Transaction<'a> {
    // Creates a new block.
    pub fn new(_to:  &'a str, _sender:  &'a str, _amount: u64, _data:  &'a str) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            to: _to,
            sender: _sender,
            amount: _amount,
            signature: "",
            data: _data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
      let mut vec = Vec::new();

      vec.extend(&convert_u64_to_u8_array(self.timestamp as u64));
      vec.extend_from_slice(self.to.as_bytes());
      vec.extend_from_slice(self.sender.as_bytes());
      vec.extend(&convert_u64_to_u8_array(self.amount));
      vec.extend_from_slice(self.data.as_bytes());

      vec
    }

    pub fn defunc_hash(&self) -> Sha256Hash {
      let tx:Vec<u8> = self.serialize();
      
      let mut hasher = Sha256::new();
      hasher.input(&tx);
      let mut hash = Sha256Hash::default();

      hasher.result(&mut hash);

      hash
    }

    pub fn add_signature(&mut self, sig: &'a str) -> &Transaction {
        if self.signature.to_string() != "".to_string() {
            panic!("Already signed tx");
        }
        self.signature = sig;
        self
    }
}

// This transforms a u64 into a little endian array of u8
pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
    return [
        val as u8,
        (val >> 8) as u8,
        (val >> 16) as u8,
        (val >> 24) as u8,
        (val >> 32) as u8,
        (val >> 40) as u8,
        (val >> 48) as u8,
        (val >> 56) as u8,
    ]
}
