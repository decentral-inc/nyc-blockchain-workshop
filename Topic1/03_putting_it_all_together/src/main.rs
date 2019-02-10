
#[macro_use]
extern crate t_bang;
use t_bang::*;

#[macro_use]
mod ecdsa;
use ecdsa::*;

mod transaction;
use transaction::Transaction;

fn main() {
    
    

    let (bob_sk, bob_pk) = ecdsa::generate();
    let (alice_sk, alice_pk) = ecdsa::generate();
    println!(
        "secret_key: {:?}, public_key:{:?}",
        bob_sk, bob_pk
    );
    let sig_str = sign(&bob_sk, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    println!("{}", verify("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &sig_str, &bob_pk));
    let data = "bob sends alice 5 btc";

    let tx: Transaction = Transaction::new(alice_pk, bob_pk, 5, data.to_string());

    
    println!("{:?}", tx.defunc_hash().len());
    
}