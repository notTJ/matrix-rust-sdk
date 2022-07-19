//! Types to handle requests.
use deno_bindgen::deno_bindgen;
// use crate::Result;
use crate::errors::DenoError;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};
use std::cmp::Ordering;
use std::convert::{self, Infallible};
use std::f64;
use std::fmt;
use std::iter::{self, Product, Sum};
use std::mem;
use std::str;
use std::str::FromStr;
use crate::UnwrapThrowExt;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use js_sys::String;
use matrix_sdk_crypto::{
    requests::{
        KeysBackupRequest as RumaKeysBackupRequest, KeysQueryRequest as RumaKeysQueryRequest,
        RoomMessageRequest as RumaRoomMessageRequest, ToDeviceRequest as RumaToDeviceRequest,
    },
    OutgoingRequests,
};
use ruma::api::client::keys::{
    claim_keys::v3::Request as RumaKeysClaimRequest,
    upload_keys::v3::Request as RumaKeysUploadRequest,
    upload_signatures::v3::Request as RumaSignatureUploadRequest,
};
// use wasm_bindgen::prelude::*;

// placeholder
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsString {
    value: String,
}

pub struct JsValue {
    idx: String, // u32,
    // _marker: marker::PhantomData<*mut u8>, // not at all threadsafe
}

// impl JsValue {
//     #[inline]
//     pub fn from_str(s: &str) -> JsValue {
//         // unsafe { JsValue::_new(__wbindgen_string_new(s.as_ptr(), s.len())) }
//         JsValue { idx: String::from(s) }
//     }
// }

// impl JsString {
//     #[inline]
//     pub fn from_str(s: &str) -> JsString {
//         // unsafe { JsValue::_new(__wbindgen_string_new(s.as_ptr(), s.len())) }
//         JsString { value: String::from(s) }
//     }
// }

impl<'a> From<&'a str> for JsString {
    fn from(s: &'a str) -> Self {
        JsString { value: String::from(s) }
        // JsString::from_str(s)
    }
}

impl From<String> for JsString {
    fn from(s: String) -> Self {
        From::from(&*s)
    }
}

impl<'a> From<&'a JsString> for String {
    fn from(s: &'a JsString) -> Self {
        return s.clone().value;
        // let val = Self::from_str(&s.value);
        // return val.into();
        // Self::from_str(s.value.as_string().unwrap_throw())
        // Self::from_str(&s.value);
    }
}

impl From<JsString> for String {
    fn from(s: JsString) -> Self {
        From::from(&s)
    }
}


impl str::FromStr for JsString {
    type Err = convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JsString::from(s))
    }
}

/// Data for a request to the `/keys/upload` API endpoint
/// ([specification]).
///
/// Publishes end-to-end encryption keys for the device.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysupload
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct KeysUploadRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"device_keys": …, "one_time_keys": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl KeysUploadRequest {
    /// Create a new `KeysUploadRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> KeysUploadRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysUpload
    }
}

/// Data for a request to the `/keys/query` API endpoint
/// ([specification]).
///
/// Returns the current devices and identity keys for the given users.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysquery
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct KeysQueryRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"timeout": …, "device_keys": …, "token": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl KeysQueryRequest {
    /// Create a new `KeysQueryRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> KeysQueryRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysQuery
    }
}

/// Data for a request to the `/keys/claim` API endpoint
/// ([specification]).
///
/// Claims one-time keys that can be used to establish 1-to-1 E2EE
/// sessions.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysclaim
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct KeysClaimRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"timeout": …, "one_time_keys": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl KeysClaimRequest {
    /// Create a new `KeysClaimRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> KeysClaimRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysClaim
    }
}

/// Data for a request to the `/sendToDevice` API endpoint
/// ([specification]).
///
/// Send an event to a single device or to a group of devices.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3sendtodeviceeventtypetxnid
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct ToDeviceRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"event_type": …, "txn_id": …, "messages": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl ToDeviceRequest {
    /// Create a new `ToDeviceRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> ToDeviceRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::ToDevice
    }
}

/// Data for a request to the `/keys/signatures/upload` API endpoint
/// ([specification]).
///
/// Publishes cross-signing signatures for the user.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keyssignaturesupload
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct SignatureUploadRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"signed_keys": …, "txn_id": …, "messages": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl SignatureUploadRequest {
    /// Create a new `SignatureUploadRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> SignatureUploadRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::SignatureUpload
    }
}

/// A customized owned request type for sending out room messages
/// ([specification]).
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3roomsroomidsendeventtypetxnid
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct RoomMessageRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"room_id": …, "txn_id": …, "content": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl RoomMessageRequest {
    /// Create a new `RoomMessageRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> RoomMessageRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::RoomMessage
    }
}

/// A request that will back up a batch of room keys to the server
/// ([specification]).
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3room_keyskeys
#[derive(Debug)]
// #[wasm_bindgen(getter_with_clone)]
pub struct KeysBackupRequest {
    /// The request ID.
    // #[wasm_bindgen(readonly)]
    pub id: JsString,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"rooms": …}
    /// ```
    // #[wasm_bindgen(readonly)]
    pub body: JsString,
}

// #[wasm_bindgen]
impl KeysBackupRequest {
    /// Create a new `KeysBackupRequest`.
    // #[wasm_bindgen(constructor)]
    pub fn new(id: JsString, body: JsString) -> KeysBackupRequest {
        Self { id, body }
    }

    /// Get its request type.
    // #[wasm_bindgen(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysBackup
    }
}

macro_rules! request {
    ($request:ident from $ruma_request:ident maps fields $( $field:ident ),+ $(,)? ) => {
        impl TryFrom<(String, &$ruma_request)> for $request {
            type Error = serde_json::Error;

            fn try_from(
                (request_id, request): (String, &$ruma_request),
            ) -> Result<Self, Self::Error> {
                let mut map = serde_json::Map::new();
                $(
                    map.insert(stringify!($field).to_owned(), serde_json::to_value(&request.$field).unwrap());
                )+
                let value = serde_json::Value::Object(map);

                Ok($request {
                    id: request_id.into(),
                    body: serde_json::to_string(&value)?.into(),
                })
            }
        }
    };
}

request!(KeysUploadRequest from RumaKeysUploadRequest maps fields device_keys, one_time_keys);
request!(KeysQueryRequest from RumaKeysQueryRequest maps fields timeout, device_keys, token);
request!(KeysClaimRequest from RumaKeysClaimRequest maps fields timeout, one_time_keys);
request!(ToDeviceRequest from RumaToDeviceRequest maps fields event_type, txn_id, messages);
request!(SignatureUploadRequest from RumaSignatureUploadRequest maps fields signed_keys);
request!(RoomMessageRequest from RumaRoomMessageRequest maps fields room_id, txn_id, content);
request!(KeysBackupRequest from RumaKeysBackupRequest maps fields rooms);

// JavaScript has no complex enums like Rust. To return structs of
// different types, we have no choice that hiding everything behind a
// `JsValue`.
pub(crate) struct OutgoingRequest(pub(crate) matrix_sdk_crypto::OutgoingRequest);

// impl TryFrom<OutgoingRequest> for JsValue {
//     type Error = serde_json::Error;

//     fn try_from(outgoing_request: OutgoingRequest) -> Result<Self, Self::Error> {
//         let request_id = outgoing_request.0.request_id().to_string();

//         Ok(match outgoing_request.0.request() {
//             OutgoingRequests::KeysUpload(request) => {
//                 JsValue::from(KeysUploadRequest::try_from((request_id, request))?)
//             }

//             OutgoingRequests::KeysQuery(request) => {
//                 JsValue::from(KeysQueryRequest::try_from((request_id, request))?)
//             }

//             OutgoingRequests::KeysClaim(request) => {
//                 JsValue::from(KeysClaimRequest::try_from((request_id, request))?)
//             }

//             OutgoingRequests::ToDeviceRequest(request) => {
//                 JsValue::from(ToDeviceRequest::try_from((request_id, request))?)
//             }

//             OutgoingRequests::SignatureUpload(request) => {
//                 JsValue::from(SignatureUploadRequest::try_from((request_id, request))?)
//             }

//             OutgoingRequests::RoomMessage(request) => {
//                 JsValue::from(RoomMessageRequest::try_from((request_id, request))?)
//             }

//             OutgoingRequests::KeysBackup(request) => {
//                 JsValue::from(KeysBackupRequest::try_from((request_id, request))?)
//             }
//         })
//     }
// }
/// Represent the type of a request.
// #[wasm_bindgen]
#[deno_bindgen]
#[derive(Debug)]
pub enum RequestType {
    /// Represents a `KeysUploadRequest`.
    KeysUpload,

    /// Represents a `KeysQueryRequest`.
    KeysQuery,

    /// Represents a `KeysClaimRequest`.
    KeysClaim,

    /// Represents a `ToDeviceRequest`.
    ToDevice,

    /// Represents a `SignatureUploadRequest`.
    SignatureUpload,

    /// Represents a `RoomMessageRequest`.
    RoomMessage,

    /// Represents a `KeysBackupRequest`.
    KeysBackup,
}
