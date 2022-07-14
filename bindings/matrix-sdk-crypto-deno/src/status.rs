use std::fmt::{Display, Formatter, Result};

// use crate::sys;

#[repr(i32)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Status {
  Ok = 0,
  InvalidArg,
  ObjectExpected,
  StringExpected,
  NameExpected,
  FunctionExpected,
  NumberExpected,
  BooleanExpected,
  ArrayExpected,
  GenericFailure,
  PendingException,
  Cancelled,
  EscapeCalledTwice,
  HandleScopeMismatch,
  CallbackScopeMismatch,
  /// ThreadSafeFunction queue is full
  QueueFull,
  /// ThreadSafeFunction closed
  Closing,
  BigintExpected,
  DateExpected,
  ArrayBufferExpected,
  DetachableArraybufferExpected,
  WouldDeadlock,
  Unknown = 1024, // unknown status. for example, using napi3 module in napi7 Node.js, and generate an invalid napi3 status
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let status_string = format!("{:?}", self);
    write!(f, "{}", status_string)
  }
}

impl From<i32> for Status {
  fn from(code: i32) -> Self {
    match code {
      NapiStatus::napi_ok => Status::Ok,
      NapiStatus::napi_invalid_arg => Status::InvalidArg,
      NapiStatus::napi_object_expected => Status::ObjectExpected,
      NapiStatus::napi_string_expected => Status::StringExpected,
      NapiStatus::napi_name_expected => Status::NameExpected,
      NapiStatus::napi_function_expected => Status::FunctionExpected,
      NapiStatus::napi_number_expected => Status::NumberExpected,
      NapiStatus::napi_boolean_expected => Status::BooleanExpected,
      NapiStatus::napi_array_expected => Status::ArrayExpected,
      NapiStatus::napi_generic_failure => Status::GenericFailure,
      NapiStatus::napi_pending_exception => Status::PendingException,
      NapiStatus::napi_cancelled => Status::Cancelled,
      NapiStatus::napi_escape_called_twice => Status::EscapeCalledTwice,
      NapiStatus::napi_handle_scope_mismatch => Status::HandleScopeMismatch,
      NapiStatus::napi_callback_scope_mismatch => Status::CallbackScopeMismatch,
      NapiStatus::napi_queue_full => Status::QueueFull,
      NapiStatus::napi_closing => Status::Closing,
      NapiStatus::napi_bigint_expected => Status::BigintExpected,
      NapiStatus::napi_date_expected => Status::DateExpected,
      NapiStatus::napi_arraybuffer_expected => Status::ArrayBufferExpected,
      NapiStatus::napi_detachable_arraybuffer_expected => Status::DetachableArraybufferExpected,
      NapiStatus::napi_would_deadlock => Status::WouldDeadlock,
      _ => Status::Unknown,
    }
  }
}

impl From<Status> for i32 {
  fn from(code: Status) -> Self {
    match code {
      Status::Ok => NapiStatus::napi_ok,
      Status::InvalidArg => NapiStatus::napi_invalid_arg,
      Status::ObjectExpected => NapiStatus::napi_object_expected,
      Status::StringExpected => NapiStatus::napi_string_expected,
      Status::NameExpected => NapiStatus::napi_name_expected,
      Status::FunctionExpected => NapiStatus::napi_function_expected,
      Status::NumberExpected => NapiStatus::napi_number_expected,
      Status::BooleanExpected => NapiStatus::napi_boolean_expected,
      Status::ArrayExpected => NapiStatus::napi_array_expected,
      Status::GenericFailure => NapiStatus::napi_generic_failure,
      Status::PendingException => NapiStatus::napi_pending_exception,
      Status::Cancelled => NapiStatus::napi_cancelled,
      Status::EscapeCalledTwice => NapiStatus::napi_escape_called_twice,
      Status::HandleScopeMismatch => NapiStatus::napi_handle_scope_mismatch,
      Status::CallbackScopeMismatch => NapiStatus::napi_callback_scope_mismatch,
      Status::QueueFull => NapiStatus::napi_queue_full,
      Status::Closing => NapiStatus::napi_closing,
      Status::BigintExpected => NapiStatus::napi_bigint_expected,
      Status::DateExpected => NapiStatus::napi_date_expected,
      Status::ArrayBufferExpected => NapiStatus::napi_arraybuffer_expected,
      Status::DetachableArraybufferExpected => NapiStatus::napi_detachable_arraybuffer_expected,
      Status::WouldDeadlock => NapiStatus::napi_would_deadlock,
      Status::Unknown => NapiStatus::napi_generic_failure,
    }
  }
}

pub type napi_status = i32;

pub mod NapiStatus {
  pub const napi_ok: i32 = 0;
  pub const napi_invalid_arg: i32 = 1;
  pub const napi_object_expected: i32 = 2;
  pub const napi_string_expected: i32 = 3;
  pub const napi_name_expected: i32 = 4;
  pub const napi_function_expected: i32 = 5;
  pub const napi_number_expected: i32 = 6;
  pub const napi_boolean_expected: i32 = 7;
  pub const napi_array_expected: i32 = 8;
  pub const napi_generic_failure: i32 = 9;
  pub const napi_pending_exception: i32 = 10;
  pub const napi_cancelled: i32 = 11;
  pub const napi_escape_called_twice: i32 = 12;
  pub const napi_handle_scope_mismatch: i32 = 13;
  pub const napi_callback_scope_mismatch: i32 = 14;
  pub const napi_queue_full: i32 = 15;
  pub const napi_closing: i32 = 16;
  pub const napi_bigint_expected: i32 = 17;
  pub const napi_date_expected: i32 = 18;
  pub const napi_arraybuffer_expected: i32 = 19;
  pub const napi_detachable_arraybuffer_expected: i32 = 20;
  pub const napi_would_deadlock: i32 = 21; // unused
}