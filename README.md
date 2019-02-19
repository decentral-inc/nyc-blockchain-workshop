# nyc-blockchain-workshop
Workshop files used for Blockchain Developer Seminar: Applied Rust for Protocol Development

We will send out finished projects after each tutorial. Code demonstration will be given live. If you prefer following along more than welcome to. We will release a video and a finalized codebase later this week. 


# Overview

To run this project, make sure we have the latest `rustc` version, which is the official compiler for Rust. 

### Dependency: Rust developer environment

Follow guide here: [https://doc.rust-lang.org/1.0.0/book/installing-rust.html](https://doc.rust-lang.org/1.0.0/book/installing-rust.html)

Verify it works after installing:     
```
Lings-MacBook-Pro:nyc-blockchain-workshop lmeng$ rustup -V
rustup 1.16.0 (beab5ac2b 2018-12-06)
Lings-MacBook-Pro:nyc-blockchain-workshop lmeng$ rustc -V
rustc 1.34.0-nightly (eac09088e 2019-02-15)
```

**IMPORTANT:** If you already have Rust, still need to update it to the latest version by running   

```bash
rustup update
```


Verify it worked:   
   
```
Lings-MacBook-Pro:nyc-blockchain-workshop lmeng$ rustup update
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: syncing channel updates for 'nightly-x86_64-apple-darwin'
info: checking for self-updates

   stable-x86_64-apple-darwin unchanged - rustc 1.32.0 (9fda7c223 2019-01-16)
  nightly-x86_64-apple-darwin unchanged - rustc 1.34.0-nightly (eac09088e 2019-02-15)
```


### Dependency: Python

If you don't have python, the easiest way to install on any OS is to go to the python website and download the official installer. If you're on OSX, go to [https://www.python.org/downloads/](https://www.python.org/downloads/) and look for the section called `Download the latest version for Mac OS X`, then hit the yellow download button.   


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




### Dependency NPM and Node

I assume most people have this but please download the latest versions if you don't.  

Verify you have it in Terminal:      
    
```  
Lings-MacBook-Pro:nyc-blockchain-workshop lmeng$ npm -v
6.4.1
Lings-MacBook-Pro:nyc-blockchain-workshop lmeng$ node -v
v11.4.0
```  

  
### Optional: Postman GUI

Postman is for testing JSONRPC endpoints. The GUI may be a better alternative to our python client (CLI) for  understanding the lower level networking level details for RPC which we'll be covering in Topic 1 section 01, 02, 03.  

Download it at [https://www.getpostman.com/apps](https://www.getpostman.com/apps)   



# Body

## I. Topic 1

for Topic 1, 

### Take a look of how ECDSA works 

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
