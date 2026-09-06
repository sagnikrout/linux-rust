//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/crashdump-ppc64.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Backup region - first 64KB of System RAM
//
// If ever the below macros are to be changed, please be judicious.
// The implicit assumptions are:
// - start, end & size are less than UINT32_MAX.
// - start & size are at least 8 byte aligned.
//
// For implementation details: arch/powerpc/purgatory/trampoline_64.S
//
pub const BACKUP_SRC_START: c_int = 0;
pub const BACKUP_SRC_END: c_uint = 0xffff;

