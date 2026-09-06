
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

use crate::parse::ParseStream;
use proc_macro2::{Delimiter, TokenStream};
use std::cmp::Ordering;
use std::iter;

pub(crate) fn between<'a>(begin: ParseStream<'a>, end: ParseStream<'a>) -> TokenStream {
    let end = end.cursor();
    let mut cursor = begin.cursor();
    assert!(crate::buffer::same_buffer(end, cursor));

    let mut tokens = TokenStream::new();
    while cursor != end {
        let (tt, next) = cursor.token_tree().unwrap();

        if crate::buffer::cmp_assuming_same_buffer(end, next) == Ordering::Less {
            // A syntax node can cross the boundary of a None-delimited group
            // due to such groups being transparent to the parser in most cases.
            // Any time this occurs the group is known to be semantically
            // irrelevant. https://github.com/dtolnay/syn/issues/1235
            if let Some((inside, _span, after)) = cursor.group(Delimiter::None) {
                assert!(next == after);
                cursor = inside;
                continue;
            } else {
                panic!("verbatim end must not be inside a delimited group");
            }
        }

        tokens.extend(iter::once(tt));
        cursor = next;
    }
    tokens
}
