# Topic 1-2: ECDSA


This tutorial teaches how ECDSA works and builds ring signatures based on rust bindings of secp256k1 C++ library which is used in Bitcoin.


# Asymmetric Cryptography


Symmetric cryptography is often compared to lock and key as

key : lock :: secret key : encryped data

This secures data from others it also certifies that the data **belongs to a person.**

However, on a network protocol where nodes are sharing data between nodes, it is meaningless to use the cryptography since everyone is able to reproduce with the shared keys.

To overcome this, we apply asymmetric cryptography, which there is one key to only encrypt the data, and the other to decrypt only.
Signing key is called private key, and verifying key for decryption is called public key.

# Eliptic Curve Digital Signature Algorithm

Eliptic Curve Digital Signature Algorithm(ECDSA) is one of asymmetric cryptography. ECDSA uses two systems to encrypt the data; eliptic curve and finite field(Galois field). 

Eliptic curve arithmetic is one-way and multiplication can be applied with repeating addition only. This makes it hard to undo operations to find the original value, which is the private key.

Finite field is a field where it contains a finite number of elements limited with a number p. Suppose p = 13 then in the finite field  
3 mod 13 = 16 mod 13 (1)
This makes it harder to find the original value of a given operation by undoing because it makes many possibilities such as (1).

for more information on how it works, check out the workshop from Scaling Bitcoin:  
![Scaling Bitcoin: ECDSA](https://youtu.be/PDzGP621pEs?t=69)


# Setup


If the project cannot be compiled, update rust with rust version manager `rustup`

```bash
rustup update
```   

Set it up to use nightly by default  
  
```bash
rustup default nightly
```
  
  
Navigate to the directory of this project from the cloned repository on terminal and build the project
```bash
cd ./Topic1/01_simple_web_server/
cargo build
```

then run the project.
  
```bash
cargo run
```

afterwards, to run executable  

```bash
 ./target/debug/rust_crypto
 ```

In this project, we have managed bits and bytes for you to easily generate private(secret)/public keys and sha256 hash from struct.

> A tuple struct  
```
struct Pair(i32, f32);
```
> A struct with two fields  
```
struct Point {
    x: f32,
    y: f32,
}
```

# Usage


## ecdsa

```rust
mod ecdsa;


let (sk, pk) = ecdsa::generate();
println!("secret_key: {}, public_key:{}", sk, pk);
/* prints secret_key: 3397b8b6faba3f83925dcffb51773a28f59530dd3cb6fccf6e3518094040ff70, public_key:022da9ebc229b9436ae89781e12b5787c5e26c3bf555e522b500443df637a9a873 */

let signature = sign_raw(&sk, txhash);
println!("{}", signature);
/* prints 304402207eaa99cac098aed4cfd7779aae6fd5e547cfaefda81383b83cc4b3a4b01defeb02201b7dc1f51093896301a674a70e0cd037567a65aa3a89066efaf1d64eea7e8d840000 */

```

## transaction

```rust

let data = "Bob sends Alice to 5 eth";
let mut tx: Transaction = Transaction::new(&alice_pk, &bob_pk, 5, data);
/* returns Transaction { timestamp: 1549893371, to: "022337a6ba0c0229fb48469bd49745b200f4cdb35459e7033dbd846bee66ee87be", sender: "02a03b99517daf92dd3925eaf02cc5b6e9a90314a70baaa22e7e5383b1580df730", amount: 5, signature: "304402202f8046faf00d945a74c0f42e7e05c7a8360ff4681d57b524c5da79bc2d2058f80220456fe85f731fa07a17361963198c47f2dfd4ee5b6ea9d9932d8a6626ba53d4fe0000", data: "Bob sends Alice to 5 eth" } */


let txhash = tx.defunc_hash();
println!("{:?}", txhash);
/* prints 32 byte array [97, 47, 223, 9, 131, 127, 59, 167, 82, 210, 232, 206, 47, 113,230, 43, 242, 9, 8, 35, 210, 158, 74, 51, 112, 152, 225, 162, 70, 229, 186, 88] */

```

