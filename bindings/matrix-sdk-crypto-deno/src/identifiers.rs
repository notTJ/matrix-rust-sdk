// // ! Types for [Matrix](https://matrix.org/) identifiers for devices,
// // ! events, keys, rooms, servers, users and URIs.

use deno_bindgen::deno_bindgen;
// use napi_derive::*;

// use crate::into_err;

/// A Matrix [user ID].
///
/// [user ID]: https://spec.matrix.org/v1.2/appendices/#user-identifiers

#[deno_bindgen]
#[derive(Debug, Clone)]
pub struct UserId {
    pub(crate) inner: ruma::OwnedUserId,
}

// impl From<ruma::OwnedUserId> for UserId {
//     fn from(inner: ruma::OwnedUserId) -> Self {
//         Self { inner }
//     }
// }

// #[deno_bindgen]
// impl UserId {
//     /// Return the user ID as a string.
//     // #[napi]
//     #[allow(clippy::inherent_to_string)]
//     pub fn to_string(&self) -> String {
//         self.inner.as_str().to_owned()
//     }
// }