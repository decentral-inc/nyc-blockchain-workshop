extern crate rand;
extern crate secp256k1;
#[macro_use]
extern crate t_bang;
use t_bang::*;

use rand::OsRng;
use secp256k1::{Message, Secp256k1};

mod ecdsa;
use ecdsa::*;



fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (sk1, pk1) = secp.generate_keypair(&mut rng);
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
    
    

    let (sk_str, pk_str) = ecdsa::generate();
    println!("{}", sk_str);
    println!("{}", pk_str);
    let sig = secp.sign(&message, &sk1);
    println!(
        "message: {:?}, secret_key: {:?}, public_key:{:?}",
        message, sk1, pk1
    );
    //println!("Encoded message: {}, secret_key: {}, public_key:{}", hex::encode(message.as_mut_ptr()), hex::encode(sk1.as_mut_ptr()), hex::encode(pk1.as_mut_ptr()));
    println!("{}", secp.verify(&message, &sig, &pk1).is_ok());
}