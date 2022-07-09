// // ! Types for [Matrix](https://matrix.org/) identifiers for devices,
// // ! events, keys, rooms, servers, users and URIs.

use deno_bindgen::deno_bindgen;
// use napi_derive::*;

use crate::{into_err, errors::Error};

/// A Matrix [user ID].
///
/// [user ID]: https://spec.matrix.org/v1.2/appendices/#user-identifiers

#[deno_bindgen]
#[derive(Debug, Clone)]
pub struct SimpleUserId {
    pub(crate) id: i32,
}

#[derive(Debug, Clone)]
pub struct UserId {
    pub(crate) inner: ruma::OwnedUserId,
}

impl From<ruma::OwnedUserId> for UserId {
    fn from(inner: ruma::OwnedUserId) -> Self {
        Self { inner }
    }
}

#[deno_bindgen]
fn hold_userId() {

}

impl UserId {
    /// Parse/validate and create a new `UserId`.
    // #[napi(constructor)]
    pub fn new(id: String) -> Result<Self, Error> {
        Ok(Self::from(ruma::UserId::parse(id.as_str()).map_err(into_err)?))
    }

    /// Return the user ID as a string.
    // #[napi]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.inner.as_str().to_owned()
    }
}