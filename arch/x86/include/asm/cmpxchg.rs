//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cmpxchg.h
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
// Non-existent functions to indicate usage errors at link time
// (or compile-time if the compiler implements __compiletime_error().
//
// Constants for operation sizes. On 32-bit, the 64-bit size it set to
// -1 because sizeof will never return -1, thereby making those switch
// case statements guaranteed dead code which the compiler will
// eliminate, and allowing the "missing symbol in the default case" to
// indicate a usage error.
//
pub const __X86_CASE_B: c_int = 1;
pub const __X86_CASE_W: c_int = 2;
pub const __X86_CASE_L: c_int = 4;

pub const __X86_CASE_Q: c_int = 8;

//
// An exchange-type operation, which takes a value and a pointer, and
// returns the old value.
//

//
// Note: no "lock" prefix even on SMP: xchg always implies lock anyway.
// Since this is generally used to protect other memory information, we
// use "asm volatile" and "memory" clobbers to prevent gcc from moving
// information around.
//

//
// Atomic compare and exchange.  Compare OLD with MEM, if identical,
// store NEW in MEM.  Return the initial value in MEM.  Success is
// indicated by comparing RETURN with OLD.
//

// _old = __old;						\

//
// xadd() adds "inc" to "*ptr" and atomically returns the previous
// value of "*ptr".
//
// xadd() is locked when multiple CPUs are online
//

