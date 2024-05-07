use starknet_devnet_server::{
    api::json_rpc::{models::BlockIdInput, JsonRpcHandler},
    rpc_core::response::ResponseResult,
    rpc_handler::RpcHandler,
};
use starknet_devnet_types::starknet_api::block::BlockNumber;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::js_callback::JsCallbackHolder;

#[derive(Clone)]
pub struct JsonRpcWrapper {
    json_rpc_handler: JsonRpcHandler,
    // TODO: alter
    js_callback: Arc<Mutex<JsCallbackHolder<serde_json::Value>>>,
    block_number: Arc<Mutex<BlockNumber>>,
}

impl JsonRpcWrapper {
    pub fn new(json_rpc_handler: JsonRpcHandler, js_callback: JsCallbackHolder<serde_json::Value>) -> Self {
        Self {
            json_rpc_handler,
            js_callback: Arc::new(Mutex::new(js_callback)),
            block_number: Arc::new(Mutex::new(BlockNumber(u64::MAX))),
        }
    }

    async fn send_block(&self, block_number: BlockNumber) {
        let block = self
            .json_rpc_handler
            .on_request(<JsonRpcWrapper as RpcHandler>::Request::BlockWithFullTransactions(
                BlockIdInput {
                    block_id: starknet_core::types::BlockId::Number(block_number.0).into(),
                },
            ))
            .await;

        let calldata = match block {
            ResponseResult::Success(val) => val,
            ResponseResult::Error(err) => {
                // TODO: change callback to have 2 argument. Error and Response
                serde_json::to_value(err).unwrap()
            }
        };

        self.js_callback.lock().await.deref_mut().call(calldata);
    }
}

#[async_trait::async_trait]
impl RpcHandler for JsonRpcWrapper {
    type Request = <JsonRpcHandler as RpcHandler>::Request;

    async fn on_request(&self, request: Self::Request) -> ResponseResult {
        let response = self.json_rpc_handler.on_request(request).await;

        let latest_block = self.json_rpc_handler.api.starknet.read().await.get_latest_block();
        let latest_block_number = match latest_block {
            Ok(block) => block.block_number(),
            // No blocks yet. Return response
            Err(_) => return response,
        };

        let mut block_number = self.block_number.lock().await;
        if *block_number != latest_block_number {
            *block_number = latest_block_number;
            self.send_block(latest_block_number).await;
        }

        response
    }
}
