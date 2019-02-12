
#[macro_use]
extern crate t_bang;
use t_bang::*;

#[macro_use]
mod ecdsa;
use ecdsa::*;

mod transaction;
use transaction::Transaction;

use jsonrpc_core::*;
use jsonrpc_http_server::*;
use serde_json::json;
use serde::{Deserialize};
use std::collections::HashMap;






fn main() {
    
    let mut io = IoHandler::new();

    io.add_method("verify_transaction", |params: Params| {
        let parsed: Transaction = params.parse().unwrap();
        let is_self = verify_raw(parsed.defunc_hash(), &parsed.signature, &parsed.sender);
        Ok(Value::String(format!("{}", is_self)))
    });

    
    
    let server = ServerBuilder::new(io)
    .start_http(&"127.0.0.1:3031".parse().unwrap())
    .expect("Unable to start RPC server");
    server.wait();
}