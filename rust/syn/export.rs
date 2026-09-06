
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: Apache-2.0 OR MIT

#[doc(hidden)]
pub use std::clone::Clone;
#[doc(hidden)]
pub use std::cmp::{Eq, PartialEq};
#[doc(hidden)]
pub use std::concat;
#[doc(hidden)]
pub use std::default::Default;
#[doc(hidden)]
pub use std::fmt::Debug;
#[doc(hidden)]
pub use std::hash::{Hash, Hasher};
#[doc(hidden)]
pub use std::marker::Copy;
#[doc(hidden)]
pub use std::option::Option::{None, Some};
#[doc(hidden)]
pub use std::result::Result::{Err, Ok};
#[doc(hidden)]
pub use std::stringify;

#[doc(hidden)]
pub type Formatter<'a> = std::fmt::Formatter<'a>;
#[doc(hidden)]
pub type FmtResult = std::fmt::Result;

#[doc(hidden)]
pub type bool = std::primitive::bool;
#[doc(hidden)]
pub type str = std::primitive::str;

#[cfg(feature = "printing")]
#[doc(hidden)]
pub use quote;

#[doc(hidden)]
pub type Span = proc_macro2::Span;
#[doc(hidden)]
pub type TokenStream2 = proc_macro2::TokenStream;

#[cfg(feature = "parsing")]
#[doc(hidden)]
pub use crate::group::{parse_braces, parse_brackets, parse_parens};

#[doc(hidden)]
pub use crate::span::IntoSpans;

#[cfg(all(feature = "parsing", feature = "printing"))]
#[doc(hidden)]
pub use crate::parse_quote::parse as parse_quote;

#[cfg(feature = "parsing")]
#[doc(hidden)]
pub use crate::token::parsing::{peek_punct, punct as parse_punct};

#[cfg(feature = "printing")]
#[doc(hidden)]
pub use crate::token::printing::punct as print_punct;

#[cfg(feature = "parsing")]
#[doc(hidden)]
pub use crate::token::private::CustomToken;

#[cfg(feature = "proc-macro")]
#[doc(hidden)]
pub type TokenStream = proc_macro::TokenStream;

#[cfg(feature = "printing")]
#[doc(hidden)]
pub use quote::{ToTokens, TokenStreamExt};

#[doc(hidden)]
pub struct private(pub(crate) ());
