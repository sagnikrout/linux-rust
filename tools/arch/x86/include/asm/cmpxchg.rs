//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/asm/cmpxchg.h
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
// Non-existant functions to indicate usage errors at link time
// (or compile-time if the compiler implements __compiletime_error().
//
// Constants for operation sizes. On 32-bit, the 64-bit size it set to
// -1 because sizeof will never return -1, thereby making those switch
// case statements guaranteeed dead code which the compiler will
// eliminate, and allowing the "missing symbol in the default case" to
// indicate a usage error.
//
pub const __X86_CASE_B: c_int = 1;
pub const __X86_CASE_W: c_int = 2;
pub const __X86_CASE_L: c_int = 4;

pub const __X86_CASE_Q: c_int = 8;

//
// Atomic compare and exchange.  Compare OLD with MEM, if identical,
// store NEW in MEM.  Return the initial value in MEM.  Success is
// indicated by comparing RETURN with OLD.
//

