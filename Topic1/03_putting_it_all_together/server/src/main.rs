
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
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
struct ValidateParams {
    certificate: String,
    public_key: String
}

fn main() {
    
    let mut io = IoHandler::new();

    io.add_method("new_account", |params: Params| {
        println!("{:?}", params);
        let (sk, pk) = ecdsa::generate();
        let v = json!({
            "secret_key": &sk,
            "public_key": &pk
        });
        Ok(serde_json::to_value(v).unwrap())
    });

    io.add_method("validate", |params: Params| {
        let parsed: ValidateParams = params.parse().ok_or_else(Error::invalid_params)?;
        println!("{:?}", parsed);
        Ok(Value::String("".to_string()))
    });


    let server = ServerBuilder::new(io)
    .start_http(&"127.0.0.1:3031".parse().unwrap())
    .expect("Unable to start RPC server");
    server.wait();
}