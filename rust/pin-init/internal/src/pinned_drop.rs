
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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Nothing, parse_quote, spanned::Spanned, ImplItem, ItemImpl, Token};

use crate::diagnostics::{DiagCtxt, ErrorGuaranteed};

pub(crate) fn pinned_drop(
    _args: Nothing,
    mut input: ItemImpl,
    dcx: &mut DiagCtxt,
) -> Result<TokenStream, ErrorGuaranteed> {
    if let Some(unsafety) = input.unsafety {
        dcx.error(unsafety, "implementing `PinnedDrop` is safe");
    }
    input.unsafety = Some(Token![unsafe](input.impl_token.span));
    match &mut input.trait_ {
        Some((not, path, _for)) => {
            if let Some(not) = not {
                dcx.error(not, "cannot implement `!PinnedDrop`");
            }
            for (seg, expected) in path
                .segments
                .iter()
                .rev()
                .zip(["PinnedDrop", "pin_init", ""])
            {
                if expected.is_empty() || seg.ident != expected {
                    dcx.error(seg, "bad import path for `PinnedDrop`");
                }
                if !seg.arguments.is_none() {
                    dcx.error(&seg.arguments, "unexpected arguments for `PinnedDrop` path");
                }
            }
            *path = parse_quote!(::pin_init::PinnedDrop);
        }
        None => {
            let span = input
                .impl_token
                .span
                .join(input.self_ty.span())
                .unwrap_or(input.impl_token.span);
            dcx.error(
                span,
                "expected `impl ... PinnedDrop for ...`, got inherent impl",
            );
        }
    }
    for item in &mut input.items {
        if let ImplItem::Fn(fn_item) = item {
            if fn_item.sig.ident == "drop" {
                fn_item
                    .sig
                    .inputs
                    .push(parse_quote!(_: ::pin_init::__internal::OnlyCallFromDrop));
            }
        }
    }
    Ok(quote!(#input))
}
