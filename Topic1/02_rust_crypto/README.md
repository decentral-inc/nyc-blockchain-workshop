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

If the project cannot be compiled, update rust with rust version manager `rustup`
```bash
rustup update
```   
  
Navigate to the directory of this project from the cloned repository on terminal and build the project
```bash
cd ./Topic1/01_simple_web_server/
cargo build
```

then run the project.
  
```
cargo run
```