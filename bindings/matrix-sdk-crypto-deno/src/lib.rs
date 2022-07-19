// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// #![doc = include_str!("../README.md")]
// #![cfg_attr(docsrs, feature(doc_auto_cfg))]
//#![warn(missing_docs, missing_debug_implementations)]
pub type Result<T> = std::result::Result<T, crate::errors::DenoError>;

pub mod encryption;
mod errors;
pub mod events;
pub mod identifiers;
pub mod machine;
pub mod requests;
pub mod responses;
pub mod sync_events;
pub mod either;
// #[cfg(feature = "tracing")]
// pub mod tracing;

use errors::DenoError;

use crate::errors::into_err;

pub trait UnwrapThrowExt<T>: Sized {
    /// Unwrap this `Option` or `Result`, but instead of panicking on failure,
    /// throw an exception to JavaScript.
    fn unwrap_throw(self) -> T {
        self.expect_throw("`unwrap_throw` failed")
    }

    /// Unwrap this container's `T` value, or throw an error to JS with the
    /// given message if the `T` value is unavailable (e.g. an `Option<T>` is
    /// `None`).
    fn expect_throw(self, message: &str) -> T;
}

impl<T> UnwrapThrowExt<T> for Option<T> {
    fn expect_throw(self, message: &str) -> T {
        if cfg!(all(target_arch = "wasm32", not(target_os = "emscripten"))) {
            match self {
                Some(val) => val,
                None => throw_str(message),
            }
        } else {
            self.expect(message)
        }
    }
}

impl<T, E> UnwrapThrowExt<T> for core::result::Result<T, E>
where
    E: core::fmt::Debug,
{
    fn expect_throw(self, message: &str) -> T {
        // if cfg!(all(target_arch = "wasm32", not(target_os = "emscripten"))) {
        //     match self {
        //         Ok(val) => val,
        //         Err(_) => throw_str(message),
        //     }
        // } else {
        //     self.expect(message)
        // }
        match self {
            Ok(val) => val,
            Err(_) => throw_str(message),
        }
    }
}


pub fn throw_str(s: &str) -> ! {
    panic!("{}", crate::errors::DenoError::from_reason(s).reason)
}
