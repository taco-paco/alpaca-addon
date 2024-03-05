use neon::context::Context;
use neon::handle::Handle;
use neon::object::Object;
use neon::result::NeonResult;
use neon::types::{Finalize, JsArray, JsObject, JsValue, Value};
use serde::Serialize;
use std::marker::PhantomData;

use crate::errors::Result;

pub trait FromJsValue {
    type Output;
    fn from_js_value<'a, C: Context<'a>>(cx: &mut C, object: Handle<'a, JsObject>) -> NeonResult<Self::Output>;
}

/// A trait for converting a rust type into JS type
pub trait IntoJsType {
    type JsType: Value + neon::handle::Managed;
    fn into_js_type<'a, C>(self, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>;
}

/// A proxy trait for implementation of IntoJsResult
pub trait IntoJsTypeProxy {
    type Type;
    type JsType: Value + neon::handle::Managed;

    fn into_js_type<'a, C>(value: Self::Type, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>;
}

pub struct BoxedResultProxy<T> {
    _p: PhantomData<T>,
}

impl<T: Send + Finalize + 'static> IntoJsTypeProxy for BoxedResultProxy<T> {
    type Type = T;
    type JsType = JsValue;

    fn into_js_type<'a, C>(value: Self::Type, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        Ok(vec![cx.boxed(value).as_value(cx)])
    }
}

pub struct JsonValueTypeProxy<T> {
    _p: PhantomData<T>,
}
impl<T: Serialize> IntoJsTypeProxy for JsonValueTypeProxy<T> {
    type Type = T;
    type JsType = JsValue;

    fn into_js_type<'a, C>(value: Self::Type, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        Ok(neon_serde2::to_value(cx, &value).map(|el| vec![el])?)
    }
}

pub struct JsArraySerializedTypeProxy<T> {
    _p: PhantomData<T>,
}
impl<T: Serialize> IntoJsTypeProxy for JsArraySerializedTypeProxy<T> {
    type Type = Vec<T>;
    type JsType = JsValue;

    fn into_js_type<'a, C>(value: Self::Type, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        let js_array = JsArray::new(cx, value.len() as u32);
        for (i, s) in value.into_iter().enumerate() {
            let v = JsonValueTypeProxy::into_js_type(s, cx)?[0];
            js_array.set(cx, i as u32, v)?;
        }

        Ok(vec![js_array.as_value(cx)])
    }
}

pub trait IntoJsTypeBlanket {
    type Proxy: IntoJsTypeProxy<Type = Self>;
}
impl<T: Send + IntoJsTypeBlanket + 'static> IntoJsType for T {
    type JsType = <<T as IntoJsTypeBlanket>::Proxy as IntoJsTypeProxy>::JsType;
    fn into_js_type<'a, C>(self, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        T::Proxy::into_js_type::<C>(self, cx)
    }
}

pub struct PromisifiedJsTypeProxy<T> {
    _p: PhantomData<T>,
}

// Promisified format: [Error, Value]
impl<T: IntoJsType<JsType = JsValue>> IntoJsTypeProxy for PromisifiedJsTypeProxy<T> {
    type Type = Result<T>;
    type JsType = JsValue;

    fn into_js_type<'a, C>(value: Self::Type, cx: &mut C) -> Result<Vec<Handle<'a, Self::JsType>>>
    where
        C: Context<'a>,
    {
        match value {
            Ok(value) => {
                let vec: Vec<Handle<'a, JsValue>> = value.into_js_type(cx)?;
                let mut res = vec![cx.undefined().as_value(cx)];
                res.extend(&vec);

                Ok(res)
            }
            Err(err) => Err(err),
        }
    }
}
