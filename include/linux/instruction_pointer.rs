//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/instruction_pointer.h
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
// The current generic definition of _THIS_IP_ is considered broken by GCC [1]
// and Clang [2]. In particular, the address of a label is only expected to be
// used with a computed goto.
//
// [1] https://gcc.gnu.org/bugzilla/show_bug.cgi?id=120071
// [2] https://github.com/llvm/llvm-project/issues/138272
//
// Mark it as broken, so that appropriate fallback options can be implemented
// for architectures that do not define their own _THIS_IP_.
//
// Macro flag: #define HAS_BROKEN_THIS_IP

//
// _CODE_LOCATION_ provides a unique identifier for the current code location.
// When _THIS_IP_ is broken (generic version), we fall back to a static marker
// which guarantees uniqueness and resolves to a constant address at link time,
// avoiding runtime overhead and compiler optimizations breaking it.
//

