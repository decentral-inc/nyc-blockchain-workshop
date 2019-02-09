extern crate rand;
extern crate secp256k1;
use rand::OsRng;
use secp256k1::{Message, Secp256k1, Signature, SecretKey};
use std::str::FromStr;
use std::convert::From::from;


// Generates secret key and public key byte array
pub fn generate() -> (String, String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (sk, pk) = secp.generate_keypair(&mut rng);
    let sk_str = sk.to_string();
    let pk_str = pk.to_string();
    return (sk_str, pk_str);
}

pub fn sign(sk_str: &str, msg_str: &str) -> String {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_str(sk_str).unwrap();
    let msg = Message::from(msg_str);
    let sig = secp.sign(&msg, &sk);
    return sig.to_string();
}
