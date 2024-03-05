use neon::context::Context;
use neon::handle::Handle;
use neon::object::Object;
use neon::prelude::{JsNumber, JsObject, JsResultExt, NeonResult};
use neon::types::{JsValue, Value};
use serde::Serialize;

use crate::{
    errors::Result,
    js_traits::{FromJsValue, IntoJsType, IntoJsTypeBlanket, JsArraySerializedTypeProxy, JsonValueTypeProxy},
};

pub struct DevnetConfig {
    pub seed: u32,
    pub total_accounts: u8,
    pub port: u16,
}

impl FromJsValue for DevnetConfig {
    type Output = Self;

    fn from_js_value<'a, C: Context<'a>>(cx: &mut C, object: Handle<'a, JsObject>) -> NeonResult<Self::Output> {
        let seed = object
            .get(cx, "seed")?
            .downcast::<JsNumber, _>(cx)
            .or_throw(cx)?
            .value(cx) as u32;
        let total_accounts = object
            .get(cx, "totalAccounts")?
            .downcast::<JsNumber, _>(cx)
            .or_throw(cx)?
            .value(cx) as u8;
        let port = object
            .get(cx, "port")?
            .downcast::<JsNumber, _>(cx)
            .or_throw(cx)?
            .value(cx) as u16;

        Ok(Self {
            seed,
            total_accounts,
            port,
        })
    }
}

impl Into<starknet_devnet_core::starknet::starknet_config::StarknetConfig> for DevnetConfig {
    fn into(self) -> starknet_devnet_core::starknet::starknet_config::StarknetConfig {
        use starknet_devnet_core::starknet::starknet_config::StarknetConfig;

        let mut config = StarknetConfig::default();
        config.port = self.port;
        config.seed = self.seed;
        config.total_accounts = self.total_accounts;

        config
    }
}

#[derive(Serialize)]
pub(crate) struct AccountData {
    pub account_address: starknet_devnet_types::contract_address::ContractAddress,
    pub public_key: starknet_devnet_types::felt::Key,
    pub private_key: starknet_devnet_types::felt::Key,
    pub balance: starknet_devnet_types::felt::Balance,
}

impl From<starknet_devnet_core::account::Account> for AccountData {
    fn from(value: starknet_devnet_core::account::Account) -> Self {
        Self {
            account_address: value.account_address,
            private_key: value.private_key,
            public_key: value.public_key,
            balance: value.initial_balance,
        }
    }
}

// Register type
impl IntoJsTypeBlanket for Vec<AccountData> {
    type Proxy = JsArraySerializedTypeProxy<AccountData>;
}

// Register type
impl IntoJsTypeBlanket for serde_json::Value {
    type Proxy = JsonValueTypeProxy<serde_json::Value>;
}

impl IntoJsType for Result<Vec<AccountData>> {
    type JsType = JsValue;
    fn into_js_type<'a, C>(self, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        match self {
            Ok(val) => {
                let mut res = vec![cx.undefined().as_value(cx)];
                res.extend(&val.into_js_type(cx)?);
                Ok(res)
            }
            Err(err) => Err(err),
        }
    }
}
