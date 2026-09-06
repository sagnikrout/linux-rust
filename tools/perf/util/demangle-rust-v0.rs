//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/demangle-rust-v0.h
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


// SPDX-License-Identifier: Apache-2.0 OR MIT
// The contents of this file come from the Rust rustc-demangle library, hosted
// in the <https://github.com/rust-lang/rustc-demangle> repository, licensed
// under "Apache-2.0 OR MIT". For copyright details, see
// <https://github.com/rust-lang/rustc-demangle/blob/main/README.md>.
// Please note that the file should be kept as close as possible to upstream.

// Macro flag: #define DEMANGLE_NODISCARD

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum demangle_style {
    DemangleStyleUnknown = 0,
    DemangleStyleLegacy,
    DemangleStyleV0,
}

// Not using a union here to make the struct easier to copy-paste if needed.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct demangle {
    pub style: demangle_style,
// points to the "mangled" part of the name,
// not including `ZN` or `R` prefixes.
    pub mangled: *const c_char,
    pub mangled_len: usize,
// In DemangleStyleLegacy, is the number of path elements
    pub elements: usize,
// while it's called "original", it will not contain `.llvm.9D1C9369@@16` suffixes
// that are to be ignored.
    pub original: *const c_char,
    pub original_len: usize,
// Contains the part after the mangled name that is to be outputted,
// which can be `.exit.i.i` suffixes LLVM sometimes adds.
    pub suffix: *const c_char,
    pub suffix_len: usize,
}

// if the length of the output buffer is less than `output_len-OVERFLOW_MARGIN`,
// the demangler will return `OverflowOverflow` even if there is no overflow.
pub const OVERFLOW_MARGIN: c_int = 4;
// Demangle a C string that refers to a Rust symbol and put the demangle intermediate result in `res`.
// Beware that `res` contains references into `s`. If `s` is modified (or free'd) before calling
// `rust_demangle_display_demangle` behavior is undefined.
//
// Use `rust_demangle_display_demangle` to convert it to an actual string.
extern "C" {
    pub fn rust_demangle_demangle(s: *const c_char, res: *mut demangle);
}
// Write the string in a `struct demangle` into a buffer.
//
// Return `OverflowOk` if the output buffer was sufficiently big, `OverflowOverflow` if it wasn't.
// This function is `O(n)` in the length of the input + *output* [$], but the demangled output of demangling a symbol can
// be exponentially[$$] large, therefore it is recommended to have a sane bound (`rust-demangle`
// uses 1,000,000 bytes) on `len`.
//
// `alternate`, if true, uses the less verbose alternate formatting (Rust `{:#}`) is used, which does not show
// symbol hashes and types of constant ints.
//
// [$] It's `O(n * MAX_DEPTH)`, but `MAX_DEPTH` is a constant 300 and therefore it's `O(n)`
// [$$] Technically, bounded by `O(n^MAX_DEPTH)`, but this is practically exponential.
extern "C" {
    pub fn rust_demangle_display_demangle(res: *const demangle, out: *mut c_char, len: usize, alternate: bool) -> DEMANGLE_NODISCARD overflow_status;
}
// Returns true if `res` refers to a known valid Rust demangling style, false if it's an unknown style.
extern "C" {
    pub fn rust_demangle_is_known(res: *mut demangle) -> bool;
}

