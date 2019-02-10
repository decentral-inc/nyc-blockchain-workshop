
#[macro_use]
extern crate t_bang;
use t_bang::*;

#[macro_use]
mod ecdsa;
use ecdsa::*;

extern crate rand;
extern crate secp256k1;
use rand::OsRng;
use secp256k1::{Message, Secp256k1, Signature, SecretKey, PublicKey};
use std::str::FromStr;


fn main() {
    
    

    let (sk_str, pk_str) = ecdsa::generate();
    
    println!(
        "secret_key: {:?}, public_key:{:?}",
        sk_str, pk_str
    );
    let sig_str = sign(&sk_str, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    println!("{}", verify("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &sig_str, &pk_str));
}