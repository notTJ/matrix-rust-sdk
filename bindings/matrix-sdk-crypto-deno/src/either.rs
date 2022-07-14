// // I don't like having this but for porting from napi here it is.

// // a bunch of this stuff may belong in deno bindings or something..
// const ERROR_MSG: &str = "The return value of typeof(T) should not be equal in Either";

// either.rs stuff
#[derive(Debug, Clone, Copy)]
pub enum Either<A, B> {
  A(A),
  B(B),
}

// impl napi raw..
pub trait RawValue {
  // #[allow(clippy::missing_safety_doc)]
  fn raw(&self) -> raw_value;
}
/// JsValue ptr
pub type raw_value = *mut raw_value__;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_value__ {
  _unused: [u8; 0],
}

impl<A: RawValue, B: RawValue> Either<A, B> {
  /// # Safety
  pub fn raw(&self) -> raw_value {
    match &self {
      Self::A(a) => a.raw(),
      Self::B(b) => b.raw(),
    }
  }
}

pub struct Null;
pub type Undefined = ();

impl<T> From<Option<T>> for Either<T, Undefined> {
  fn from(value: Option<T>) -> Self {
    match value {
      Some(v) => Either::A(v),
      None => Either::B(()),
    }
  }
}


impl<T> From<Either<T, Null>> for Option<T> {
  fn from(value: Either<T, Null>) -> Option<T> {
    match value {
      Either::A(v) => Some(v),
      Either::B(_) => None,
    }
  }
}
// Typename shenanigans :s
pub trait TypeName {
  fn type_name() -> &'static str;

  fn value_type() -> ValueType;
}


// other

// value_type.rs
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum ValueType {
  Undefined = 0,
  Null = 1,
  Boolean = 2,
  Number = 3,
  String = 4,
  Symbol = 5,
  Object = 6,
  Function = 7,
  External = 8,
//   #[cfg(feature = "napi6")]
  BigInt = 9,
  Unknown = 1024,
}

// the either macros
macro_rules! either_n {
  ( $either_name:ident, $( $parameter:ident ),+ $( , )* ) => {
    #[derive(Debug, Clone, Copy)]
    pub enum $either_name< $( $parameter ),+ > {
      $( $parameter ( $parameter ) ),+
    }

    // impl< $( $parameter ),+ > TypeName for $either_name < $( $parameter ),+ >
    //   where $( $parameter: TypeName ),+
    // {
    //   fn type_name() -> &'static str {
    //     stringify!( $either_name )
    //   }

    //   fn value_type() -> ValueType {
    //     ValueType::Unknown
    //   }
    // }


  }
}
// macro_rules! either_n {
//     ( $either_name:ident, $( $parameter:ident ),+ $( , )* ) => {
//       #[derive(Debug, Clone, Copy)]
//       pub enum $either_name< $( $parameter ),+ > {
//         $( $parameter ( $parameter ) ),+
//       }
  
//       impl< $( $parameter ),+ > TypeName for $either_name < $( $parameter ),+ >
//         where $( $parameter: TypeName ),+
//       {
//         fn type_name() -> &'static str {
//           stringify!( $either_name )
//         }
  
//         fn value_type() -> ValueType {
//           ValueType::Unknown
//         }
//       }
  
//       impl< $( $parameter ),+ > FromNapiValue for $either_name < $( $parameter ),+ >
//         where $( $parameter: TypeName + FromNapiValue + ValidateNapiValue ),+
//       {
//         unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> crate::Result<Self> {
//           $(
//             if unsafe { $parameter::validate(env, napi_val).is_ok() } {
//               unsafe { $parameter ::from_napi_value(env, napi_val).map(Self:: $parameter ) }
//             } else
//           )+
//           {
//             Err(crate::Error::new(
//               Status::InvalidArg,
//               format!(
//                 concat!("Value is non of these types ", $( "`{", stringify!( $parameter ), "}`, " ),+ ),
//                 $( $parameter = $parameter::value_type(), )+
//               ),
//             ))
//           }
//         }
//       }
  
//       impl< $( $parameter ),+ > ToNapiValue for $either_name < $( $parameter ),+ >
//         where $( $parameter: ToNapiValue ),+
//       {
//         unsafe fn to_napi_value(
//           env: sys::napi_env,
//           value: Self
//         ) -> crate::Result<crate::sys::napi_value> {
//           match value {
//             $( Self:: $parameter (v) => unsafe { $parameter ::to_napi_value(env, v) } ),+
//           }
//         }
//       }
//     };
//   }

// either_n!(Either3, A, B, C);
// either_n!(Either4, A, B, C, D);
// either_n!(Either5, A, B, C, D, E);
// either_n!(Either6, A, B, C, D, E, F);
// either_n!(Either7, A, B, C, D, E, F, G);
// either_n!(Either8, A, B, C, D, E, F, G, H);
// either_n!(Either9, A, B, C, D, E, F, G, H, I);
// either_n!(Either10, A, B, C, D, E, F, G, H, I, J);
// either_n!(Either11, A, B, C, D, E, F, G, H, I, J, K);
// either_n!(Either12, A, B, C, D, E, F, G, H, I, J, K, L);
// either_n!(Either13, A, B, C, D, E, F, G, H, I, J, K, L, M);
// either_n!(Either14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
// either_n!(Either15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
// either_n!(Either16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
// either_n!(Either17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
// either_n!(Either18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
// either_n!(Either19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
// either_n!(Either20, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
// either_n!(Either21, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
// either_n!(Either22, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
// either_n!(Either23, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
// either_n!(Either24, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
// either_n!(Either25, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
// either_n!(Either26, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
