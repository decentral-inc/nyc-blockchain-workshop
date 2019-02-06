
# Topic 1: simple_web_server
=============================

This tutorial instructs how to build a simple web RPC server and test it with clients, and it uses [JSON-RPC 2.0](https://www.jsonrpc.org/specification) specification to exchange data.

# Why JSON-RPC 2.0?
=====================
JSON-RPC is a light-weight remote procedure call (RPC) protocol. Its lightness attracted many blockchain projects to adopt this to their protocol.

Blockchain projects using this specification are:
- Parity(they even implemented this specification by themselves in Rust)
- Ethereum
- Bitcoin
- Ripple
...and many more

# Setup
========

To setup the project, simply run `cargo new simple_web_server` on the shell.

and setup dependancies like this in `Cargo.toml`.

```toml

[dependencies]
jsonrpc-core = "10.0.1"
jsonrpc-http-server = "10.0.1"
jsonrpc-derive = "10.0.2"

```

then run `cargo build`


# Client

Client sends request to RPC server and it is implemented in blockchain and wallets.

It sends two types of params


as an array:
```json
{
	"jsonrpc": "2.0",
	"method": "say_hello",
	"params": ["world"],
	"id":1
}
```

or as an object:
```json
{
	"jsonrpc": "2.0",
	"method": "say_hello",
	"params": {"name": "Hyungsuk Kang"},
	"id":1
}
```

Make API with the interface to send corresponding json to RPC endpoints then you get:
- web3.js
- web3.py
- web3j
- web3swift
- polkadot-api

For tutorial we will use Postman, but you can also test your server with the client file built [here](./client/client.py)

### Once you install postman set the request options as shown as below:
![postman setting1](https://i.imgur.com/VeuI1Ip.png)

![postman setting2](https://i.imgur.com/qvXnAtA.png)


Press send button and you have now finished a RPC-JSON server development!



