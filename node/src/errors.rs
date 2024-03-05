use neon::context::Context;
use neon::handle::Handle;
use neon::object::Object;
use neon::types::{JsError, JsValue};

use crate::js_traits::IntoJsType;

#[derive(Clone, Copy)]
enum ErrorType {
    Internal = 0,
    Devnet,
}

impl From<ErrorType> for u32 {
    fn from(error_type: ErrorType) -> Self {
        match error_type {
            ErrorType::Internal => 0,
            ErrorType::Devnet => 1,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DevnetStarknetError(#[from] starknet_devnet_core::error::Error),

    #[error(transparent)]
    DevnetServerError(#[from] starknet_devnet_server::error::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Neon error")]
    JsError(neon::result::Throw),

    #[error("")]
    NeonSerdeError(#[from] neon_serde2::errors::Error),
}

impl From<neon::result::Throw> for Error {
    fn from(value: neon::result::Throw) -> Error {
        Error::JsError(value)
    }
}

impl IntoJsType for Error {
    type JsType = JsValue;
    fn into_js_type<'a, C>(self, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        // TODO: add backtrace
        struct Info {
            error_type: u32,
            details: String,
        }

        // TODO: introduce ErrorType
        let info = match &self {
            Error::DevnetStarknetError(err) => Info {
                error_type: ErrorType::Devnet.into(),
                details: err.to_string(),
            },
            Error::IoError(err) => Info {
                error_type: ErrorType::Internal.into(),
                details: err.to_string(),
            },
            Error::JsError(err) => Info {
                error_type: ErrorType::Internal.into(),
                details: err.to_string(),
            },
            Error::NeonSerdeError(err) => Info {
                error_type: ErrorType::Internal.into(),
                details: err.to_string(),
            },
            Error::DevnetServerError(err) => Info {
                error_type: ErrorType::Devnet.into(),
                details: err.to_string(),
            }
        };

        let error = match JsError::error(cx, info.details) {
            Ok(val) => val,
            Err(_) => return Ok(vec![cx.string("Failed to create an JsError").upcast()]),
        };

        let error_type = cx.number(info.error_type);
        if let Err(_) = error.set(cx, "type", error_type) {
            return Ok(vec![cx.string("Failed to set error type o").upcast()]);
        }

        let engine = cx.string("alpaca-addon");
        if let Err(_) = error.set(cx, "tag", engine) {
            return Ok(vec![cx
                .string("Failed to set engine type of async task error")
                .upcast()]);
        }

        Ok(vec![error.upcast()])
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
