use jsonrpc_core::*;
use jsonrpc_http_server::*;
fn main() {
    let mut io = IoHandler::new();
    io.add_method("say_hello", |_params: Params| {
        let params: Value = _params.parse().unwrap();
        Ok(Value::String(format!("{}, {}","hello".to_string(), params["name"].to_string())))
    });

    let _server = ServerBuilder::new(io)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .expect("Unable to start RPC server");

_server.wait();
}
