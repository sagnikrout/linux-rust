//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/sigcontext.h
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
//
// S390 version
// Copyright IBM Corp. 1999, 2000
//

pub const __NUM_GPRS: c_int = 16;
pub const __NUM_FPRS: c_int = 16;
pub const __NUM_ACRS: c_int = 16;
pub const __NUM_VXRS: c_int = 32;
pub const __NUM_VXRS_LOW: c_int = 16;
pub const __NUM_VXRS_HIGH: c_int = 16;
// Has to be at least _NSIG_WORDS from asm/signal.h
pub const _SIGCONTEXT_NSIG: c_int = 64;
pub const _SIGCONTEXT_NSIG_BPW: c_int = 64;
// Size of stack frame allocated when calling signal handler.
pub const __SIGNAL_FRAMESIZE: c_int = 160;

