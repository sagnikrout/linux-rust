//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/div64.h
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
// do_div() is NOT a C function. It wants to return
// two values (the quotient and the remainder), but
// since that doesn't work very well in C, what it
// does is:
//
// - modifies the 64-bit dividend _in_place_
// - returns the 32-bit remainder
//
// This ends up being the most efficient "calling
// convention" on x86.
//

//
// gcc tends to zero extend 32bit values and do full 64bit maths.
// Define asm functions that avoid this.
// (clang generates better code for the C versions.)
//

//
// __div64_32() is never called on x86, so prevent the
// generic definition from getting built.
//
// Macro flag: #define __div64_32

//
// Will generate an #DE when the result doesn't fit u64, could fix with an
// __ex_table[] entry when it becomes an issue.
//

extern "C" {
    pub fn mul_u64_add_u64_div_u64(_arg: a, _arg: mul, _arg: 0, _arg: div) -> return;
}

