//! Automatically rewritten from C to Rust
//! Source: rust/exports.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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
//
// A hack to export Rust symbols for loadable modules without having to redo
// the entire `include/linux/export.h` logic in Rust.
//
// This requires Rust's new/future `v0` mangling scheme because the default one
// ("legacy") uses invalid characters for C identifiers (thus we cannot use the
// `EXPORT_SYMBOL_*` macros).
//
// All symbols are exported as GPL-only to guarantee no GPL-only feature is
// accidentally exposed.
//

// For modules using `rust/build_error.rs`.

    EXPORT_SYMBOL_RUST_GPL(rust_build_error);
