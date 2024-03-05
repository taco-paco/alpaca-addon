use axum::routing::{get, post};
use starknet_devnet_core::starknet::starknet_config::StarknetConfig;
use starknet_devnet_server::{
    api::http::endpoints as http,
    builder::StarknetDevnetServer,
    rpc_handler::RpcHandler,
    {builder, ServerConfig},
};
use std::net::SocketAddr;

use crate::errors::Result;

/// Configures an [axum::Server] that handles related JSON-RPC calls and WEB API calls via HTTP
pub fn serve_http_api_json_rpc<TJsonRpcHandler: RpcHandler, THttpApiHandler: Clone + Send + Sync + 'static>(
    addr: SocketAddr,
    config: ServerConfig,
    json_rpc_handler: TJsonRpcHandler,
    http_api_handler: THttpApiHandler,
    starknet_config: &StarknetConfig,
) -> Result<StarknetDevnetServer> {
    Ok(builder::Builder::new(addr, json_rpc_handler, http_api_handler)
        .set_config(config)
        .json_rpc_route("/")
        .json_rpc_route("/rpc")
        .http_api_route("/is_alive", get(http::is_alive))
        .http_api_route("/dump", post(http::dump_load::dump))
        .http_api_route("/load", post(http::dump_load::load))
        .http_api_route("/postman/load_l1_messaging_contract", post(http::postman::postman_load))
        .http_api_route("/postman/flush", post(http::postman::postman_flush))
        .http_api_route(
            "/postman/send_message_to_l2",
            post(http::postman::postman_send_message_to_l2),
        )
        .http_api_route(
            "/postman/consume_message_from_l2",
            post(http::postman::postman_consume_message_from_l2),
        )
        .http_api_route("/create_block", post(http::blocks::create_block))
        .http_api_route("/abort_blocks", post(http::blocks::abort_blocks))
        .http_api_route("/restart", post(http::restart))
        .http_api_route("/set_time", post(http::time::set_time))
        .http_api_route("/increase_time", post(http::time::increase_time))
        .http_api_route("/predeployed_accounts", get(http::accounts::get_predeployed_accounts))
        .http_api_route("/account_balance", get(http::accounts::get_account_balance))
        .http_api_route("/fee_token", get(http::mint_token::get_fee_token))
        .http_api_route("/mint", post(http::mint_token::mint))
        .http_api_route("/fork_status", get(http::get_fork_status))
        .build(starknet_config)?)
}
