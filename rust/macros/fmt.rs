
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

use std::collections::BTreeSet;

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote_spanned;

/// Please see [`crate::fmt`] for documentation.
pub(crate) fn fmt(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let first_opt = input.next();
    let first_owned_str;
    let mut names = BTreeSet::new();
    let first_span = {
        let Some((mut first_str, first_span)) = (match first_opt.as_ref() {
            Some(TokenTree::Literal(first_lit)) => {
                first_owned_str = first_lit.to_string();
                Some(first_owned_str.as_str()).and_then(|first| {
                    let first = first.strip_prefix('"')?;
                    let first = first.strip_suffix('"')?;
                    Some((first, first_lit.span()))
                })
            }
            _ => None,
        }) else {
            return first_opt.into_iter().chain(input).collect();
        };

        // Parse `identifier`s from the format string.
        //
        // See https://doc.rust-lang.org/std/fmt/index.html#syntax.
        while let Some((_, rest)) = first_str.split_once('{') {
            first_str = rest;
            if let Some(rest) = first_str.strip_prefix('{') {
                first_str = rest;
                continue;
            }
            if let Some((name, rest)) = first_str.split_once('}') {
                first_str = rest;
                let name = name.split_once(':').map_or(name, |(name, _)| name);
                if !name.is_empty() && !name.chars().all(|c| c.is_ascii_digit()) {
                    names.insert(name);
                }
            }
        }
        first_span
    };

    let adapter = quote_spanned!(first_span => ::kernel::fmt::Adapter);

    let mut args = TokenStream::from_iter(first_opt);
    {
        let mut flush = |args: &mut TokenStream, current: &mut TokenStream| {
            let current = std::mem::take(current);
            if !current.is_empty() {
                let (lhs, rhs) = (|| {
                    let mut current = current.into_iter();
                    let mut acc = TokenStream::new();
                    while let Some(tt) = current.next() {
                        // Split on `=` only once to handle cases like `a = b = c`.
                        if matches!(&tt, TokenTree::Punct(p) if p.as_char() == '=') {
                            names.remove(acc.to_string().as_str());
                            // Include the `=` itself to keep the handling below uniform.
                            acc.extend([tt]);
                            return (Some(acc), current.collect::<TokenStream>());
                        }
                        acc.extend([tt]);
                    }
                    (None, acc)
                })();
                args.extend(quote_spanned!(first_span => #lhs #adapter(&(#rhs))));
            }
        };

        let mut current = TokenStream::new();
        for tt in input {
            match &tt {
                TokenTree::Punct(p) if p.as_char() == ',' => {
                    flush(&mut args, &mut current);
                    &mut args
                }
                _ => &mut current,
            }
            .extend([tt]);
        }
        flush(&mut args, &mut current);
    }

    for name in names {
        let name = Ident::new(name, first_span);
        args.extend(quote_spanned!(first_span => , #name = #adapter(&#name)));
    }

    quote_spanned!(first_span => ::core::format_args!(#args))
}
