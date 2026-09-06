
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

// SPDX-License-Identifier: GPL-2.0

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{
        Parse,
        ParseStream, //
    },
    Attribute,
    Error,
    LitStr,
    Result, //
};

/// A string literal that is required to have ASCII value only.
pub(crate) struct AsciiLitStr(LitStr);

impl Parse for AsciiLitStr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let s: LitStr = input.parse()?;
        if !s.value().is_ascii() {
            return Err(Error::new_spanned(s, "expected ASCII-only string literal"));
        }
        Ok(Self(s))
    }
}

impl ToTokens for AsciiLitStr {
    fn to_tokens(&self, ts: &mut TokenStream) {
        self.0.to_tokens(ts);
    }
}

impl AsciiLitStr {
    pub(crate) fn value(&self) -> String {
        self.0.value()
    }
}

pub(crate) fn file() -> String {
    #[cfg(not(CONFIG_RUSTC_HAS_SPAN_FILE))]
    {
        proc_macro::Span::call_site()
            .source_file()
            .path()
            .to_string_lossy()
            .into_owned()
    }

    #[cfg(CONFIG_RUSTC_HAS_SPAN_FILE)]
    {
        proc_macro::Span::call_site().file()
    }
}

/// Obtain all `#[cfg]` attributes.
pub(crate) fn gather_cfg_attrs(attr: &[Attribute]) -> impl Iterator<Item = &Attribute> + '_ {
    attr.iter().filter(|a| a.path().is_ident("cfg"))
}
