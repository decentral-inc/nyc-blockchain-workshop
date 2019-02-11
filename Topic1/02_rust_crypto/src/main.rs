extern crate secp256k1;
extern crate rand;


use rand::OsRng;
use secp256k1::{Secp256k1, Message, };

#[macro_use]
mod ecdsa;
use ecdsa::*;

mod transaction;
use transaction::Transaction;


fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng"); 
    let (sk1, pk1) = secp.generate_keypair(&mut rng);
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

    let sig = secp.sign(&message, &sk1);
    println!("message: {:?}, secret_key: {:?}, public_key:{:?}", message, sk1, pk1); 
    println!("{}",secp.verify(&message, &sig, &pk1).is_ok());
    let tx: Transaction = Transaction::new(alice_pk, bob_pk, 5, data.to_string());
}
    
