//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crc/x86/crc-pclmul-template.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Macros for accessing the [V]PCLMULQDQ-based CRC functions that are
// instantiated by crc-pclmul-template.S
//
// Copyright 2025 Google LLC
//
// Author: Eric Biggers <ebiggers@google.com>
//

//
// Call a [V]PCLMULQDQ optimized CRC function if the data length is at least 16
// bytes, the CPU has PCLMULQDQ support, and the current context may use SIMD.
//
// 16 bytes is the minimum length supported by the [V]PCLMULQDQ functions.
// There is overhead associated with kernel_fpu_begin() and kernel_fpu_end(),
// varying by CPU and factors such as which parts of the "FPU" state userspace
// has touched, which could result in a larger cutoff being better.  Indeed, a
// larger cutoff is usually better for a *single* message.  However, the
// overhead of the FPU section gets amortized if multiple FPU sections get
// executed before returning to userspace, since the XSAVE and XRSTOR occur only
// once.  Considering that and the fact that the [V]PCLMULQDQ code is lighter on
// the dcache than the table-based code is, a 16-byte cutoff seems to work well.
//

