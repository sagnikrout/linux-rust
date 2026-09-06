//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/frame.h
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
// These are stack frame creation macros.  They should be used by every
// callable non-leaf asm function to make kernel stack traces more reliable.
//

//
// This is a sneaky trick to help the unwinder find pt_regs on the stack.  The
// frame pointer is replaced with an encoded pointer to pt_regs.  The encoding
// is just setting the LSB, which makes it an invalid stack address and is also
// a signal to the unwinder that it's a pt_regs pointer in disguise.
//
// NOTE: This macro must be used *after* PUSH_AND_CLEAR_REGS because it corrupts
// the original rbp.
//

//
// This is a sneaky trick to help the unwinder find pt_regs on the stack.  The
// frame pointer is replaced with an encoded pointer to pt_regs.  The encoding
// is just clearing the MSB, which makes it an invalid stack address and is also
// a signal to the unwinder that it's a pt_regs pointer in disguise.
//
// NOTE: This macro must be used *after* SAVE_ALL because it corrupts the
// original ebp.
//

// Macro flag: #define ENCODE_FRAME_POINTER

// Macro flag: #define FRAME_BEGIN
// Macro flag: #define FRAME_END
pub const FRAME_OFFSET: c_int = 0;

