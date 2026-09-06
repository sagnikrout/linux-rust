
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

use crate::ToTokens;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Span, TokenStream};

// Not public API other than via the syn crate. Use syn::spanned::Spanned.
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Span;
}

impl Spanned for Span {
    fn __span(&self) -> Span {
        *self
    }
}

impl Spanned for DelimSpan {
    fn __span(&self) -> Span {
        self.join()
    }
}

impl<T: ?Sized + ToTokens> Spanned for T {
    fn __span(&self) -> Span {
        join_spans(self.into_token_stream())
    }
}

fn join_spans(tokens: TokenStream) -> Span {
    let mut iter = tokens.into_iter().map(|tt| tt.span());

    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };

    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}

mod private {
    use crate::ToTokens;
    use proc_macro2::extra::DelimSpan;
    use proc_macro2::Span;

    pub trait Sealed {}
    impl Sealed for Span {}
    impl Sealed for DelimSpan {}
    impl<T: ?Sized + ToTokens> Sealed for T {}
}
