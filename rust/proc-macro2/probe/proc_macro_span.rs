
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

// This code exercises the surface area that we expect of Span's unstable API.
// If the current toolchain is able to compile it, then proc-macro2 is able to
// offer these APIs too.

#![cfg_attr(procmacro2_build_probe, feature(proc_macro_span))]

extern crate proc_macro;

use core::ops::{Range, RangeBounds};
use proc_macro::{Literal, Span};
use std::path::PathBuf;

pub fn byte_range(this: &Span) -> Range<usize> {
    this.byte_range()
}

pub fn start(this: &Span) -> Span {
    this.start()
}

pub fn end(this: &Span) -> Span {
    this.end()
}

pub fn line(this: &Span) -> usize {
    this.line()
}

pub fn column(this: &Span) -> usize {
    this.column()
}

pub fn file(this: &Span) -> String {
    this.file()
}

pub fn local_file(this: &Span) -> Option<PathBuf> {
    this.local_file()
}

pub fn join(this: &Span, other: Span) -> Option<Span> {
    this.join(other)
}

pub fn subspan<R: RangeBounds<usize>>(this: &Literal, range: R) -> Option<Span> {
    this.subspan(range)
}

// Include in sccache cache key.
#[cfg(procmacro2_build_probe)]
const _: Option<&str> = option_env!("RUSTC_BOOTSTRAP");
