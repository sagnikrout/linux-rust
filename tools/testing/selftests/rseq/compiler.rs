//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/rseq/compiler.h
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


// SPDX-License-Identifier: LGPL-2.1-only OR MIT
//
// rseq/compiler.h
//
// Work-around asm goto compiler bugs.
//
// (C) Copyright 2021 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//
// gcc prior to 4.8.2 miscompiles asm goto.
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=58670
//
// gcc prior to 8.1.0 miscompiles asm goto at O1.
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=103908
//
// clang prior to version 13.0.1 miscompiles asm goto at O2.
// https://github.com/llvm/llvm-project/issues/52735
//
// Work around these issues by adding a volatile inline asm with
// memory clobber in the fallthrough after the asm goto and at each
// label target.  Emit this for all compilers in case other similar
// issues are found in the future.
//

// Combine two tokens.

//
// Use C11 _Generic to express unqualified type from expression. This removes
// volatile qualifier from expression type.
//

