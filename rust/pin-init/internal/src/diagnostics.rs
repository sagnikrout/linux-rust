
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

use std::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, Error};

pub(crate) struct DiagCtxt(TokenStream);
pub(crate) struct ErrorGuaranteed(());

impl DiagCtxt {
    pub(crate) fn error(&mut self, span: impl Spanned, msg: impl Display) -> ErrorGuaranteed {
        let error = Error::new(span.span(), msg);
        self.0.extend(error.into_compile_error());
        ErrorGuaranteed(())
    }

    pub(crate) fn warn(&mut self, span: impl Spanned, msg: impl Display) {
        // Have the message start on a new line for visual clarity.
        let msg = format!("\n{}", msg);
        self.0.extend(quote_spanned!(span.span() =>
            // Approximate using deprecated warning while `proc_macro_diagnostic` is unstable.
            const _: () = {
                #[deprecated = #msg]
                const fn warn() {}
                warn();
            };
        ));
    }

    pub(crate) fn with(
        fun: impl FnOnce(&mut DiagCtxt) -> Result<TokenStream, ErrorGuaranteed>,
    ) -> TokenStream {
        let mut dcx = Self(TokenStream::new());
        match fun(&mut dcx) {
            Ok(mut stream) => {
                stream.extend(dcx.0);
                stream
            }
            Err(ErrorGuaranteed(())) => dcx.0,
        }
    }
}
