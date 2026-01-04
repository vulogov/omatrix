use jsonrpc_core::*;
use jsonrpc_http_server::*;

pub fn init_api(io: &mut IoHandler) {
    log::debug!("Initializing JSON/RPC API version v1");
    io.add_method("v1/ping", |_| {
        log::debug!("Received ping request on JSON/RPC server");
        Ok(Value::String("pong".to_string()))
    });
    io.add_method("v1/version.server", |_params: Params| {
        Ok(Value::String(env!("CARGO_PKG_VERSION").into()))
    });
}
