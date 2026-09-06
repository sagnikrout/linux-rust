
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

use proc_macro2::{
    Ident,
    TokenStream,
    TokenTree, //
};
use syn::{
    parse::{
        Parse,
        ParseStream, //
    },
    Result,
    Token, //
};

pub(crate) struct Input {
    a: Ident,
    _comma: Token![,],
    b: Ident,
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            a: input.parse()?,
            _comma: input.parse()?,
            b: input.parse()?,
        })
    }
}

pub(crate) fn concat_idents(Input { a, b, .. }: Input) -> TokenStream {
    let res = Ident::new(&format!("{a}{b}"), b.span());
    TokenStream::from_iter([TokenTree::Ident(res)])
}
