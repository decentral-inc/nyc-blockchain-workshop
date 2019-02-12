extern crate rand;
extern crate secp256k1;
use rand::OsRng;
use secp256k1::{Message, Secp256k1, Signature, SecretKey, PublicKey};
use std::str::FromStr;

macro_rules! hex {
        ($hex:expr) => ({
            let mut result = vec![0; $hex.len() / 2];
            from_hex($hex, &mut result).expect("valid hex string");
            result
        });
    }

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
    let msg_bytes32 = str_to_bytes32(msg_str);
    let msg = Message::from_slice(&msg_bytes32[..]).unwrap();
    let mut sig = secp.sign(&msg, &sk);
    sig.normalize_s();
    return sig.to_string();
}

pub fn verify(msg_str: &str, sig_str: &str, pk_str: &str) -> bool {
    let secp = Secp256k1::new();
    let msg_bytes32 = str_to_bytes32(msg_str);
    let msg = Message::from_slice(&msg_bytes32[..]).unwrap();
    let byte_str = hex!(sig_str);
    secp.verify(&msg, &Signature::from_der_lax(&byte_str).unwrap(), &PublicKey::from_str(pk_str).unwrap()).is_ok()
}


pub fn str_to_bytes32(_str:&str) -> &[u8]{
    
    let _bytes32 = _str.as_bytes();
    return _bytes32;
}

pub fn from_hex(hex: &str, target: &mut [u8]) -> Result<usize, ()> {
    if hex.len() % 2 == 1 || hex.len() > target.len() * 2 {
        return Err(());
    }

    let mut b = 0;
    let mut idx = 0;
    for c in hex.bytes() {
        b <<= 4;
        match c {
            b'A'...b'F' => b |= c - b'A' + 10,
            b'a'...b'f' => b |= c - b'a' + 10,
            b'0'...b'9' => b |= c - b'0',
            _ => return Err(()),
        }
        if (idx & 1) == 1 {
            target[idx / 2] = b;
            b = 0;
        }
        idx += 1;
    }
    Ok(idx / 2)
}

pub fn sign_raw(sk_str: &str, msg: [u8; 32]) -> String {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_str(sk_str).unwrap();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let mut sig = secp.sign(&msg, &sk);
    sig.normalize_s();
    return sig.to_string();
}

pub fn verify_raw(msg: [u8; 32], sig_str: &str, pk_str: &str) -> bool {
    let secp = Secp256k1::new();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let byte_str = hex!(sig_str);
    secp.verify(&msg, &Signature::from_der_lax(&byte_str).unwrap(), &PublicKey::from_str(pk_str).unwrap()).is_ok()
}