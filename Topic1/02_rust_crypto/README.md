# Topic 1-2: ECDSA
====================

This tutorial teaches how ECDSA works and builds ring signatures based on rust bindings of secp256k1 C++ library which is used in Bitcoin.


# Asymmetric Cryptography
=========================

Symmetric cryptography is often compared to lock and key as

key : lock :: secret key : encryped data

This secures data from others it also certifies that the data **belongs to a person.**

However, on a network protocol where nodes are sharing data between nodes, it is meaningless to use the cryptography since everyone is able to reproduce with the shared keys.

To overcome this, we apply asymmetric cryptography, which there is one key to only encrypt the data, and the other to decrypt only.
Signing key is called private key, and verifying key for decryption is called public key.

# Eliptic Curve Digital Signature Algorithm
=============================================

Eliptic Curve Digital Signature Algorithm(ECDSA) is one of asymmetric cryptography. ECDSA uses two systems to encrypt the data; eliptic curve and finite field(Galois field). 

Eliptic curve arithmetic is one-way and multiplication can be applied with repeating addition only. This makes it hard to undo operations to find the original value, which is the private key.

Finite field is a field where it contains a finite number of elements limited with a number p. Suppose p = 13 then in the finite field  
3 mod 13 = 16 mod 13 (1)
This makes it harder to find the original value of a given operation by undoing because it makes many possibilities such as (1).

for more information on how it works, check out the workshop from Scaling Bitcoin:  
![Scaling Bitcoin: ECDSA](https://youtu.be/PDzGP621pEs?t=69)




# Setup
========

Update rust with rust version manager `rustup`
```bash
rustup update
```

To setup the project, simply run `cargo new simple_web_server` on the shell.

and setup dependancies like this in `Cargo.toml`.

```toml

[dependencies]
jsonrpc-core = "10.0.1"
jsonrpc-http-server = "10.0.1"
jsonrpc-derive = "10.0.2"

```

copy and paste the code below in main.rs file

```rust
extern crate secp256k1;
#[cfg(feature="rand")]
extern crate rand;


use rand::OsRng;
use secp256k1::{Secp256k1, Message, };


fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng"); 
    let (sk1, pk1) = secp.generate_keypair(&mut rng);
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

    let sig = secp.sign(&message, &sk1);
    println!("message: {:?}, secret_key: {:?}, public_key:{:?}", message, sk1, pk1); 
    println!("{}",secp.verify(&message, &sig, &pk1).is_ok());
}
```

then run `cargo build`

