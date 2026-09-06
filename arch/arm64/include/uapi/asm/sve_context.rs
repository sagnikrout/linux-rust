//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/uapi/asm/sve_context.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Copyright (C) 2017-2018 ARM Limited
//
// For use by other UAPI headers only.
// Do not make direct use of header or its definitions.
//

//
// Yes, __SVE_VQ_MAX is 512 QUADWORDS.
//
// To help ensure forward portability, this is much larger than the
// current maximum value defined by the SVE architecture.  While arrays
// or static allocations can be sized based on this value, watch out!
// It will waste a surprisingly large amount of memory.
//
// Dynamic sizing based on the actual runtime vector length is likely to
// be preferable for most purposes.
//
pub const __SVE_VQ_MIN: c_int = 1;
pub const __SVE_VQ_MAX: c_int = 512;

pub const __SVE_NUM_ZREGS: c_int = 32;
pub const __SVE_NUM_PREGS: c_int = 16;

pub const __SVE_ZREGS_OFFSET: c_int = 0;

