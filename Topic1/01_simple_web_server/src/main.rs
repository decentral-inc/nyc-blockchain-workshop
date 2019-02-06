use jsonrpc_core::*;
use jsonrpc_http_server::*;


fn main() {
    let mut io = IoHandler::new();

    io.add_method("say_hello", |params: Params| {
	let parsed: Value = params.parse().unwrap();
	Ok(Value::String(format!("hello, {}", parsed["name"])))
    });

    let _server = ServerBuilder::new(io)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .expect("Unable to start RPC server");

_server.wait();
}
