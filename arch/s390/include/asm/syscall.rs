//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/syscall.h
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
// Access to user system call parameters and results
//
// Copyright IBM Corp. 2008
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//
pub const _ASM_SYSCALL_H: c_int = 1;

//
// Unlike syscall_get_nr(), syscall_set_nr() can be called only when
// the target task is stopped for tracing on entering syscall, so
// there is no need to have the same check syscall_get_nr() has.
//
// Macro flag: #define SYSCALL_FMT_0

// Macro flag: #define SYSCALL_PARM_0

// Macro flag: #define SYSCALL_REGS_0

