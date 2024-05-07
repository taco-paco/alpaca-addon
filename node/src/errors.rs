use crate::js_traits::IntoJsType;
use neon::context::Context;
use neon::handle::Handle;
use neon::object::Object;
use neon::types::{JsError, JsValue};
use snafu::{Backtrace, ErrorCompat, Snafu};

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

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(context(false))]
    DevnetStarknetError {
        source: starknet_devnet_core::error::Error,
        backtrace: Backtrace,
    },

    #[snafu(context(false))]
    DevnetServerError {
        source: starknet_devnet_server::error::Error,
        backtrace: Backtrace,
    },

    #[snafu(context(false))]
    IoError {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Neon throw"))]
    JsError {
        error: neon::result::Throw,
        backtrace: Backtrace,
    },

    #[snafu(context(false))]
    NeonSerdeError {
        source: neon_serde2::errors::Error,
        backtrace: Backtrace,
    },
}

impl From<neon::result::Throw> for Error {
    fn from(value: neon::result::Throw) -> Error {
        JsSnafu { error: value }.build()
    }
}

struct Info {
    error_type: u32,
    details: String,
    backtrace: String,
}

impl From<&Error> for Info {
    fn from(value: &Error) -> Self {
        let backtrace = if let Some(backtrace) =  ErrorCompat::backtrace(value) {
            format!("{:?}", backtrace)
        } else {
            "<empty>".into()
        };

        match value {
            Error::DevnetStarknetError { source, backtrace: _ } => Info {
                error_type: ErrorType::Devnet.into(),
                details: source.to_string(),
                backtrace,
            },
            Error::IoError { source, backtrace: _ } => Info {
                error_type: ErrorType::Internal.into(),
                details: source.to_string(),
                backtrace,
            },
            Error::JsError { error, backtrace: _ } => Info {
                error_type: ErrorType::Internal.into(),
                details: error.to_string(),
                backtrace,
            },
            Error::NeonSerdeError { source, backtrace: _ } => Info {
                error_type: ErrorType::Internal.into(),
                details: source.to_string(),
                backtrace,
            },
            Error::DevnetServerError { source, backtrace: _ } => Info {
                error_type: ErrorType::Devnet.into(),
                details: source.to_string(),
                backtrace,
            },
        }
    }
}

impl IntoJsType for Error {
    type JsType = JsValue;
    fn into_js_type<'a, C>(self, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        let info: Info = (&self).into();
        let error = match JsError::error(cx, info.details) {
            Ok(val) => val,
            Err(_) => return Ok(vec![cx.string("Failed to create an JsError").upcast()]),
        };

        let error_type = cx.number(info.error_type);
        if let Err(_) = error.set(cx, "type", error_type) {
            return Ok(vec![cx.string("Failed to set error type").upcast()]);
        }

        let backtrace = cx.string(info.backtrace);
        if let Err(_) = error.set(cx, "backtrace", backtrace) {
            return Ok(vec![cx.string("Failed to set backtrace").upcast()]);
        }

        let engine = cx.string("alpaca-addon");
        if let Err(_) = error.set(cx, "tag", engine) {
            return Ok(vec![cx.string("Failed to set engine type").upcast()]);
        }

        Ok(vec![error.upcast()])
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
