extern crate rand;
extern crate secp256k1;
use rand::OsRng;
use secp256k1::{Message, Secp256k1, Signature, SecretKey, PublicKey};
use std::str::FromStr;

/// macro which produces hexadecimal byte array from a string
macro_rules! hex {
        ($hex:expr) => ({
            let mut result = vec![0; $hex.len() / 2];
            from_hex($hex, &mut result).expect("valid hex string");
            result
        });
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
/// * `msg_hash`: a message hash hexstring pointer 
/// # Example 
/// ```
/// use ecdsa::sign;
/// use std::str;
/// let msg = vec![97, 47, 223, 9, 131, 127, 59, 167, 82, 210, 232, 206, 47, 113,230, 43, 242, 9, 8, 35, 210, 158, 74, 51, 112, 152, 225, 162, 70, 229, 186, 88];
/// let msg_str = str::from_utf8(&msg).unwrap();
/// let signature = sign("3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70".msg_str);
/// ```
pub fn sign(sk_str: &str, msg_hash: &str) -> String {
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
/// * `msg_hash`: a message hash hexstring
/// * `sig_str`: a string poimter for secret key
/// * `pk_str` : a public key string pointer for verification 
/// # Example 
/// ```
/// use ecdsa::verify;
/// let cert = verify("","304402207eaa99cac098aed4cfd7779aae6fd5e547cfaefda81383b83cc4b3a4b01defeb02201b7dc1f51093896301a674a70e0cd037567a65aa3a89066efaf1d64eea7e8d840000","022da9ebc229b9436ae89781e12b5787c5e26c3bf555e522b500443df637a9a873");
/// ```
pub fn verify(msg_hash: &str, sig_str: &str, pk_str: &str) -> bool {
    let secp = Secp256k1::new();
    let msg_bytes32 = str_to_bytes(msg_hash);
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
/// let msg_bytes32 = str_to_bytes(msg_hash);
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
/// let signature = sign("3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70"."hello world");
/// ```
pub fn sign_raw(sk_str: &str, msg: [u8; 32]) -> String {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_str(sk_str).unwrap();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let mut sig = secp.sign(&msg, &sk);
    sig.normalize_s();
    return sig.to_string();
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
/// let msghash = str_to_bytes(msg_str);
/// let is_self = verify(msghash,"304402207eaa99cac098aed4cfd7779aae6fd5e547cfaefda81383b83cc4b3a4b01defeb02201b7dc1f51093896301a674a70e0cd037567a65aa3a89066efaf1d64eea7e8d840000","022da9ebc229b9436ae89781e12b5787c5e26c3bf555e522b500443df637a9a873");
/// ```
pub fn verify_raw(msg: [u8; 32], sig_str: &str, pk_str: &str) -> bool {
    let secp = Secp256k1::new();
    let msg = Message::from_slice(&msg[..]).unwrap();
    let byte_str = hex!(sig_str);
    secp.verify(&msg, &Signature::from_der_lax(&byte_str).unwrap(), &PublicKey::from_str(pk_str).unwrap()).is_ok()
}