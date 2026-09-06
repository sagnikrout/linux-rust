//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/libbpf_common.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Common user-facing libbpf helpers.
//
// Copyright (c) 2019 Facebook
//

// Mark a symbol as deprecated when libbpf version is >= {major}.{minor}

// Add checks for other versions below when planning deprecation of API symbols
// with the LIBBPF_DEPRECATED_SINCE macro.
//

// Macro flag: #define __LIBBPF_MARK_DEPRECATED_1_0(X)

// This set of internal macros allows to do "function overloading" based on
// number of arguments provided by used in backwards-compatible way during the
// transition to libbpf 1.0
// It's ugly but necessary evil that will be cleaned up when we get to 1.0.
// See bpf_prog_load() overload for example.
//

// Helper macro to declare and initialize libbpf options struct
//
// This dance with uninitialized declaration, followed by memset to zero,
// followed by assignment using compound literal syntax is done to preserve
// ability to use a nice struct field initialization syntax and **hopefully
// have all the padding bytes initialized to zero. It's not guaranteed though,
// when copying literal, that compiler won't copy garbage in literal's padding
// bytes, but that's the best way I've found and it seems to work in practice.
//
// Macro declares opts struct of given type and name, zero-initializes,
// including any extra padding, it with memset() and then assigns initial
// values provided by users in struct initializer-syntax as varargs.
//

// Helper macro to clear and optionally reinitialize libbpf options struct
//
// Small helper macro to reset all fields and to reinitialize the common
// structure size member. Values provided by users in struct initializer-
// syntax as varargs can be provided as well to reinitialize options struct
// specific members.
//

