extern crate rand;
extern crate secp256k1;
use rand::OsRng;
use secp256k1::{Message, Secp256k1, Signature, SecretKey, PublicKey, RecoveryId, RecoverableSignature, Error};
use std::str::{FromStr};
use std::{error};
extern crate hex_slice;
use hex_slice::AsHex;

type Err = Error;

/// macro which produces hexadecimal byte array from a string
macro_rules! hex {
        ($hex:expr) => ({
            let mut result = vec![0; $hex.len() / 2];
            from_hex($hex, &mut result).expect("valid hex string");
            result
        });
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
  strs.connect("")
}

/// Returns secret key and public key hexstring
/// # Example 
/// ```
/// use ecdsa::generate;
/// let (sk, pk) = generate();
/// ```
pub fn generate() -> (String, String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (sk, pk) = secp.generate_keypair(&mut rng);
    let sk_str = sk.to_string();
    let pk_str = pk.to_string();
    return (sk_str, pk_str);
}

/// Returns signed signature in hexstring
/// # Arguments
/// * `sk_str`: a string poimter for secret key
/// * `msg_str`: a message hash hexstring pointer 
/// # Example 
/// ```
/// use ecdsa::sign;
/// use std::str;
/// let msg = vec![97, 47, 223, 9, 131, 127, 59, 167, 82, 210, 232, 206, 47, 113,230, 43, 242, 9, 8, 35, 210, 158, 74, 51, 112, 152, 225, 162, 70, 229, 186, 88];
/// let msg_str = str::from_utf8(&msg).unwrap();
/// let signature = sign("3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70".msg_str);
/// ```
pub fn sign(sk_str: &str, msg_str: &str) -> String {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_str(sk_str).unwrap();
    let msg_bytes32 = str_to_bytes(msg_str);
    let msg = Message::from_slice(&msg_bytes32[..]).unwrap();
    let mut sig = secp.sign(&msg, &sk);
    sig.normalize_s();
    return sig.to_string();
}

/// Returns verified result in ecdsa
/// # Arguments
/// * `msg_str`: a message hash hexstring
/// * `sig_str`: a string poimter for secret key
/// * `pk_str` : a public key string pointer for verification 
/// # Example 
/// ```
/// use ecdsa::verify;
/// let msg = vec![97, 47, 223, 9, 131, 127, 59, 167, 82, 210, 232, 206, 47, 113,230, 43, 242, 9, 8, 35, 210, 158, 74, 51, 112, 152, 225, 162, 70, 229, 186, 88];
/// let msg_str = str::from_utf8(&msg).unwrap();
/// let signature = verify(msg_str,"304402207eaa99cac098aed4cfd7779aae6fd5e547cfaefda81383b83cc4b3a4b01defeb02201b7dc1f51093896301a674a70e0cd037567a65aa3a89066efaf1d64eea7e8d840000","022da9ebc229b9436ae89781e12b5787c5e26c3bf555e522b500443df637a9a873");
/// ```
pub fn verify(msg_str: &str, sig_str: &str, pk_str: &str) -> bool {
    let secp = Secp256k1::new();
    let msg_bytes32 = str_to_bytes(msg_str);
    let msg = Message::from_slice(&msg_bytes32[..]).unwrap();
    let byte_str = hex!(sig_str);
    secp.verify(&msg, &Signature::from_der_lax(&byte_str).unwrap(), &PublicKey::from_str(pk_str).unwrap()).is_ok()
}

/// Returns byte array from string
/// # Arguments
/// * `str`: a string pointer to convert to byte array
/// # Example
/// ```
/// use ecdsa::str_to_bytes;
/// let msg_str = "ddacf"
/// let msg_bytes = str_to_bytes(msg_str);
/// ```
pub fn str_to_bytes(str:&str) -> &[u8]{   
    let bytes = str.as_bytes();
    return bytes;
}


/// Returns hexstring with Result
/// # Arguments
/// * `hex` : a string to convert to hexadecimal byte array
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

/// Returns signed signature in hexstring
/// # Arguments
/// * `sk_str`: a string poimter for secret key
/// * `msg`: a message hash byte array pointer 
/// # Example 
/// ```
/// use ecdsa::sign;
/// let signature = sign_raw("3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70"."hello world");
/// ```
pub fn sign_raw(sk_str: &str, msg: [u8; 32]) -> String {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_str(sk_str).unwrap();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let mut sig = secp.sign(&msg, &sk);
    sig.normalize_s();
    return sig.to_string();
}

/// Returns signed recoverable signature in hexstring
/// # Arguments
/// * `sk_str`: a string poimter for secret key
/// * `msg`: a message hash byte array pointer 
/// # Example 
/// ```
/// use ecdsa::sign;
/// let signature = sign_raw_recoverable("3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70".[23,1213,42,123,12,32, .. 21]);
/// ```
pub fn sign_raw_recoverable(sk_str: &str, msg: [u8; 32]) -> (RecoveryId, String) {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_str(sk_str).unwrap();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let mut sig = secp.sign_recoverable(&msg, &sk);
    let compact_sig = sig.serialize_compact();
    let sig_str = to_hex_string(compact_sig.1.to_vec());
    return (compact_sig.0, String::from(sig_str));
}


/// Returns recovered public key in ecdsa
/// # Arguments
/// * `msg`: a message hash byte array
/// * `sig_str`: a string poimter for secret key
/// * `recid` : a recovery id from recoverable signature
/// # Example 
/// ```
/// use ecdsa::{verify, str_to_bytes};
/// let msg_str = // message hash hex string
/// let msg = str_to_bytes(msg_str);
/// let is_self = verify_raw(msghash,"304402207eaa99cac098aed4cfd7779aae6fd5e547cfaefda81383b83cc4b3a4b01defeb02201b7dc1f51093896301a674a70e0cd037567a65aa3a89066efaf1d64eea7e8d840000","022da9ebc229b9436ae89781e12b5787c5e26c3bf555e522b500443df637a9a873");
/// ```
pub fn ecrecover(msg: [u8; 32], sig_str: &str, recid: RecoveryId) -> Result<PublicKey, Error> {
    let sig_bytes = hex!(sig_str);
    let rec_sig = RecoverableSignature::from_compact(&sig_bytes, recid).expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");
    let msg = Message::from_slice(&msg[..]).unwrap();
    let secp = Secp256k1::new();
    secp.recover(&msg, &rec_sig)
}

/// Returns verified result in ecdsa
/// # Arguments
/// * `msg`: a message hash byte array
/// * `sig_str`: a string poimter for secret key
/// * `pk_str` : a public key string pointer for verification 
/// # Example 
/// ```
/// use ecdsa::{verify, str_to_bytes};
/// let msg_str = // message hash hex string
/// let msg = str_to_bytes(msg_str);
/// let is_self = verify_raw(msghash,"304402207eaa99cac098aed4cfd7779aae6fd5e547cfaefda81383b83cc4b3a4b01defeb02201b7dc1f51093896301a674a70e0cd037567a65aa3a89066efaf1d64eea7e8d840000","022da9ebc229b9436ae89781e12b5787c5e26c3bf555e522b500443df637a9a873");
/// ```
pub fn verify_raw(msg: [u8; 32], sig_str: &str, pk_str: &str) -> bool {
    let secp = Secp256k1::new();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let byte = hex!(sig_str);
    secp.verify(&msg, &Signature::from_der_lax(&byte).unwrap(), &PublicKey::from_str(pk_str).unwrap()).is_ok()
}