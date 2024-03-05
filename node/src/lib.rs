use neon::prelude::*;

mod devnet_adapter;
mod errors;
mod js_callback;
mod js_traits;
mod json_rpc_wrapper;
mod server_builder;
mod types;

register_module!(mut cx, {
    devnet_adapter::DevnetAdapter::export(&mut cx)?;

    Ok(())
});
