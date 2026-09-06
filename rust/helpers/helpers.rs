//! Automatically rewritten from C to Rust
//! Source: rust/helpers/helpers.c
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
// Non-trivial C macros cannot be used in Rust. Similarly, inlined C functions
// cannot be called either. This file explicitly creates functions ("helpers")
// that wrap those so that they can be called from Rust.
//
// Sorted alphabetically.
//

// Omit `inline` for bindgen as it ignores inline functions.
// Macro flag: #define __rust_helper

// The helper functions are all inline functions.
//
// We use `__always_inline` here to bypass LLVM inlining checks, in case the
// helpers are inlined directly into Rust CGUs.
//
// The LLVM inlining checks are false positives:
// * LLVM doesn't want to inline functions compiled with
// `-fno-delete-null-pointer-checks` with code compiled without.
// The C CGUs all have this enabled and Rust CGUs don't. Inlining is okay
// since this is one of the hardening features that does not change the ABI,
// and we shouldn't have null pointer dereferences in these helpers.
// * LLVM doesn't want to inline functions with different list of builtins. C
// side has `-fno-builtin-wcslen`; `wcslen` is not a Rust builtin, so they
// should be compatible, but LLVM does not perform inlining due to attributes
// mismatch.
// * clang and Rust doesn't have the exact target string. Clang generates
// `+cmov,+cx8,+fxsr` but Rust doesn't enable them (in fact, Rust will
// complain if `-Ctarget-feature=+cmov,+cx8,+fxsr` is used). x86-64 always
// enable these features, so they are in fact the same target string, but
// LLVM doesn't understand this and so inlining is inhibited. This can be
// bypassed with `--ignore-tti-inline-compatible`, but this is a hidden
// option.

