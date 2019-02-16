# nyc-blockchain-workshop
Workshop files used for Blockchain Developer Seminar: Applied Rust for Protocol Development

We will send out finished projects after each tutorial. Code demonstration will be given live. If you prefer following along more than welcome to. We will release a video and a finalized codebase later this week. 


# Overview

To run this project, make sure we have the latest `rustc` version, which is the official compiler for Rust. 

### Dependency: Rustc

Update to the latest version by running   

```bash
rustup update
```
### Dependency: Python

A python client is located in the [client folder](./client/). It will be our means of interacting with our rust server.   


To run the client, install request package from pip and locate to the client folder.  
```bash
pip install request
cd ./client/
```

and run the python code.
```bash
python client.py
# should print "true"

```

# Body

## I. Topic 1

for Topic 1, 

### Take a look of how ECDSA works in  

### Build a simple web authentication server with jsonrpc and secp256k1

### Bring it together by combining our modules in steps I and II

[Projects](./Topic1)

## II. Topic 2

### Build and run a `WASM module` in react web app

### Build and run a `runtime` (analogous to Smart Contract in the Polkadot ecosystem) 

### Deploy it on Substrate

[Projects](./Topic2)

## III. Topic 3

### Future implications to the blockchain ecosystem that Rust and WASM brings

[Projects](./Topic3)



# Conclusion