//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/access_ok.h
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
// Checking whether a pointer is valid for user space access.
// These definitions work on most architectures, but overrides can
// be used where necessary.
//
// architectures with compat tasks have a variable TASK_SIZE and should
// override this to a constant.
//

//
// 'size' is a compile-time constant for most callers, so optimize for
// this case to turn the check into a single comparison against a constant
// limit and catch all possible overflows.
// On architectures with separate user address space (m68k, s390, parisc,
// sparc64) or those without an MMU, this should always return true.
//
// This version was originally contributed by Jonas Bonn for the
// OpenRISC architecture, and was found to be the most efficient
// for constant 'size' and 'limit' values.
//

