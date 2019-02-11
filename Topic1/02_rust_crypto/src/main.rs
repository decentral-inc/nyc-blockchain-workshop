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
    // Make accounts
    let (bob_sk, bob_pk) = generate();
    let (alice_sk, alice_pk) = generate();
    // Make transaction
    let data = "Bob sends Alice to 5 eth";
    let mut tx: Transaction = Transaction::new(&alice_pk, &bob_pk, 5, data);

    // make hash out of transaction to represent it
    let txhash = tx.defunc_hash();

    // Generate certificate by signing txhash
    let certificate = sign_raw(&bob_sk, txhash);

    // Add signature for submitting to server as json
    tx.add_signature(&certificate);

    // Verify tx that the tx and its message is generated from bob
    let is_bob = verify_raw(txhash, &certificate, &bob_pk);
    let is_alice = verify_raw(txhash, &certificate, &alice_pk);
    
    println!("{:?} is generated by Bob: {}", tx, is_bob);
    println!("{:?} is generated by Bob: {}", tx, is_alice);

}
    
