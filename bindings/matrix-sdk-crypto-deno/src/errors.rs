/// Generic error wrapping `napi::Error`.
// #[derive(Debug)]
// pub struct Error(napi::Error);

// this code was copied from the nodejs bindings.
// perhaps unecessary, for now it mayt be convenient to express everything
// in 'matrix-sdk-deno' errors in case ~other errors~ need to be handled from other libs or something
#[derive(Debug)]
pub struct Error {
    pub(crate) message: String,
}

impl<E> From<E> for Error
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        // convert from lib error (deno bindings or something, napi did this)
        let error_from_reason = Error::from_reason_internal(error.to_string());
        return Self {
            message: error_from_reason.message,
        };
    }
}

impl Error {
    // this is made public to simulate an error type created by something like napi (ie deno_bindgen)
    fn from_reason_internal(reason: String) -> Error {
        
        let error = Error { message: String::from(reason) };
        return error;
    }

    pub fn from_reason(reason: &str) -> Error {
        Self::from_reason_internal(reason.to_string())
    }
}

// impl From<Error> {
//     #[inline]
//     fn from(value: Error) -> Self {
//         value.0
//     }
// }

/// Helper to replace the `E` to `Error` to `napi::Error` conversion.
#[inline]
pub fn into_err<E>(error: E) -> Error
where
    E: std::error::Error,
{
    Error::from(error).into()
}

// pub type Result = Result<Self, Error>;
