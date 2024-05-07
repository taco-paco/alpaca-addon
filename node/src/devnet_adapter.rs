use neon::prelude::*;
use neon::result::Throw;
use starknet_devnet_core::starknet::{starknet_config::StarknetConfig, Starknet};
use starknet_devnet_server::{
    api::{http::HttpApiHandler, json_rpc::JsonRpcHandler, Api},
    builder::StarknetDevnetServer,
    ServerConfig,
};
use std::net::SocketAddr;

use crate::{
    errors::Result,
    js_callback::JsCallbackHolder,
    js_traits::FromJsValue,
    json_rpc_wrapper::JsonRpcWrapper,
    server_builder::serve_http_api_json_rpc,
    types::{AccountData, DevnetConfig},
};

pub struct DevnetAdapter;

impl DevnetAdapter {
    pub fn export(cx: &mut ModuleContext) -> NeonResult<()> {
        cx.export_function("createDevnetServer", DevnetAdapter::create_devnet_server)
    }

    fn create_starknet(config: DevnetConfig) -> Result<Starknet> {
        let starknet_config: StarknetConfig = config.into();
        let mut starknet = Starknet::new(&starknet_config)?;
        if let Some(start_time) = starknet_config.start_time {
            starknet.set_block_timestamp_shift(start_time as i64 - Starknet::get_unix_timestamp_as_seconds() as i64);
        };

        Ok(starknet)
    }

    // Has to be created within tokio rt
    fn create_server_wrapper(
        starknet: Starknet,
        datafeed_callback: JsCallbackHolder<serde_json::Value>,
    ) -> Result<StarknetDevnetServer> {
        let config = starknet.config.clone();

        let api = Api::new(starknet);
        let json_rpc_handler = JsonRpcHandler { api: api.clone() };
        let http_handler = HttpApiHandler { api: api.clone() };

        let addr: SocketAddr = SocketAddr::new(config.host, config.port);
        let json_rpc_wrapper = JsonRpcWrapper::new(json_rpc_handler, datafeed_callback);
        let server = serve_http_api_json_rpc(addr, ServerConfig::default(), json_rpc_wrapper, http_handler, &config)?;

        Ok(server)
    }

    fn extract_promisified_callback(cx: &mut FunctionContext) -> Result<JsCallbackHolder<Result<Vec<AccountData>>>> {
        let callback = cx.argument::<JsFunction>(0)?.root(cx);
        let channel = cx.channel();
        Ok(JsCallbackHolder::new(callback, channel))
    }

    fn extract_datafeed_callback(cx: &mut FunctionContext) -> Result<JsCallbackHolder<serde_json::Value>> {
        let callback = cx.argument::<JsFunction>(2)?.root(cx);
        let channel = cx.channel();
        Ok(JsCallbackHolder::<serde_json::Value>::new(callback, channel))
    }

    fn extract_args(
        cx: &mut FunctionContext,
    ) -> Result<(
        JsCallbackHolder<Result<Vec<AccountData>>>,
        JsCallbackHolder<serde_json::Value>,
        DevnetConfig,
    )> {
        let config = cx.argument::<JsObject>(1)?;
        let config = DevnetConfig::from_js_value(cx, config)?;

        let promisified_callback = Self::extract_promisified_callback(cx)?;
        let datafeed_callback = Self::extract_datafeed_callback(cx)?;

        Ok((promisified_callback, datafeed_callback, config))
    }

    fn create_devnet_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let (mut promisified_callback, datafeed_callback, config) = match Self::extract_args(&mut cx) {
            Ok(val) => val,
            Err(_) => return JsResult::Err(Throw {}),
        };

        let starknet = match Self::create_starknet(config) {
            Ok(val) => val,
            Err(err) => {
                promisified_callback.call(Result::<Vec<AccountData>>::Err(err));
                return Ok(cx.undefined());
            }
        };

        std::thread::spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(err) => {
                    promisified_callback.call(Result::<Vec<AccountData>>::Err(err.into()));
                    return;
                }
            };

            rt.block_on(async move {
                let predeployed_accounts = starknet.get_predeployed_accounts();

                // Has to be created within tokio env
                let server = match Self::create_server_wrapper(starknet, datafeed_callback) {
                    Ok(server) => server,
                    Err(err) => {
                        promisified_callback.call(Result::<Vec<AccountData>>::Err(err));
                        return;
                    }
                };

                {
                    let predeployed_accounts = predeployed_accounts
                        .into_iter()
                        .map(AccountData::from)
                        .collect::<Vec<AccountData>>();

                    promisified_callback.call(Ok(predeployed_accounts));
                }

                // spawn the server on a new task
                tokio::spawn(server).await.ok();
            });
        });

        Ok(cx.undefined())
    }
}
