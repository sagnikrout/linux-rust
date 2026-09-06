//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/asm.h
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
// Helper macros to be used for flag output operand handling.
// Inline assemblies must use four of the five supplied macros:
//
// Use CC_IPM(sym) at the end of the inline assembly; this extracts the
// condition code and program mask with the ipm instruction and writes it to
// the variable with symbolic name [sym] if the compiler has no support for
// flag output operands. If the compiler has support for flag output operands
// this generates no code.
//
// Use CC_OUT(sym, var) at the output operand list of an inline assembly. This
// defines an output operand with symbolic name [sym] for the variable
// [var]. [var] must be an int variable and [sym] must be identical with [sym]
// used with CC_IPM().
//
// Use either CC_CLOBBER or CC_CLOBBER_LIST() for the clobber list. Use
// CC_CLOBBER if the clobber list contains only "cc", otherwise use
// CC_CLOBBER_LIST() and add all clobbers as argument to the macro.
//
// Use CC_TRANSFORM() to convert the variable [var] which contains the
// extracted condition code. If the condition code is extracted with ipm, the
// [var] also contains the program mask. CC_TRANSFORM() moves the condition
// code to the two least significant bits and sets all other bits to zero.
//

pub const __HAVE_ASM_FLAG_OUTPUTS__: c_int = 1;
// Macro flag: #define CC_IPM(sym)

// Macro flag: #define CC_CLOBBER

