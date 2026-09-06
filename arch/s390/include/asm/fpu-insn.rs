//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/fpu-insn.h
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
// Support for Floating Point and Vector Instructions
//

//
// Various small helper functions, which can and should be used within
// kernel fpu code sections. Each function represents only one floating
// point or vector instruction (except for helper functions which require
// exception handling).
//
// This allows to use floating point and vector instructions like C
// functions, which has the advantage that all supporting code, like
// e.g. loops, can be written in easy to read C code.
//
// Each of the helper functions provides support for code instrumentation,
// like e.g. KASAN. Therefore instrumentation is also covered automatically
// when using these functions.
//
// In order to ensure that code generated with the helper functions stays
// within kernel fpu sections, which are guarded with kernel_fpu_begin()
// and kernel_fpu_end() calls, each function has a mandatory "memory"
// barrier.
//
// fpu_lfpc_safe - Load floating point control register safely.
// @fpc: new value for floating point control register
//
// Load floating point control register. This may lead to an exception,
// since a saved value may have been modified by user space (ptrace,
// signal return, kvm registers) to an invalid value. In such a case
// set the floating point control register to zero.
//

