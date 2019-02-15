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
    /// Creates a new transaction
    /// # Arguments
    /// * `to`: address(derived from public key or itself) to send asset
    /// * `sender`: address(derived from public key or itself) who sends the asset
    /// * `amount`: amount of asset to send
    /// * `signature`: signature of the transaction
    /// * `data`: reference or specific byte code that is included in transaction. ethereum uses this for executing smart contracts.
    /// # Example
    /// ```
    /// use transaction::Transaction;
    /// let mut tx: Transaction = Transaction::new(&alice_pk, &bob_pk, 5, data);
    /// ```
    pub fn new(to:  &'a str, sender:  &'a str, amount: u64, data:  &'a str) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            to: to,
            sender: sender,
            amount: amount,
            signature: "",
            data: data,
        }
    }

    /// Returns serialized byte array from a transaction
    /// # Example
    /// ```
    /// use transaction::Transaction;
    /// let mut tx: Transaction = Transaction::new(&alice_pk, &bob_pk, 5, data);
    /// let serialized: Vec<u8> = tx.serialize();
    /// ```
    pub fn serialize(&self) -> Vec<u8> {
      let mut vec = Vec::new();

      vec.extend(&convert_u64_to_u8_array(self.timestamp as u64));
      vec.extend_from_slice(self.to.as_bytes());
      vec.extend_from_slice(self.sender.as_bytes());
      vec.extend(&convert_u64_to_u8_array(self.amount));
      vec.extend_from_slice(self.data.as_bytes());

      vec
    }

    /// Returns generated sha256 hash 32 byte array from transaction 
    /// # Example
    /// ```
    /// use transaction::Transaction;
    /// let mut tx: Transaction = Transaction::new(&alice_pk, &bob_pk, 5, data);
    /// let hash: Vec<u8> = tx.hash();
    /// ```
    pub fn hash(&self) -> Sha256Hash {
      let tx:Vec<u8> = self.serialize();
      
      let mut hasher = Sha256::new();
      hasher.input(&tx);
      let mut hash = Sha256Hash::default();

      hasher.result(&mut hash);

      hash
    }

    /// Adds signature to transaction
    /// # Argument
    /// * `sig`: signature to add in hexstring pointer
    /// # Example
    /// ```
    /// use transaction::Transaction;
    /// use ecdsa::sign;
    /// let mut tx: Transaction = Transaction::new(&alice_pk, &bob_pk, 5, data);
    /// let hash: Vec<u8> = tx.hash();
    /// let signature = sign(hash, "3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70");
    /// let signed_tx = tx.add_signature();
    /// ```
    pub fn add_signature(&mut self, sig: &'a str) -> &Transaction {
        if self.signature.to_string() != "".to_string() {
            panic!("Already signed tx");
        }
        self.signature = sig;
        self
    }
}

// Returns a little endian array of u8 from a transaction
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
