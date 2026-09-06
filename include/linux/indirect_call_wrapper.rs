//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/indirect_call_wrapper.h
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
// INDIRECT_CALL_$NR - wrapper for indirect calls with $NR known builtin
// @f: function pointer
// @f$NR: builtin functions names, up to $NR of them
// @__VA_ARGS__: arguments for @f
//
// Avoid retpoline overhead for known builtin, checking @f vs each of them and
// eventually invoking directly the builtin function. The functions are checked
// in the given order. Fallback to the indirect call.
//

// Macro flag: #define INDIRECT_CALLABLE_SCOPE

// Macro flag: #define INDIRECT_CALLABLE_DECLARE(f)

// Macro flag: #define EXPORT_INDIRECT_CALLABLE(f)

//
// We can use INDIRECT_CALL_$NR for ipv6 related functions only if ipv6 is
// builtin, this macro simplify dealing with indirect calls with only ipv4/ipv6
// alternatives
//

