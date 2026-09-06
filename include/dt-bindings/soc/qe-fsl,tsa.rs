//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/soc/qe-fsl,tsa.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
pub const FSL_QE_TSA_NU: c_int = 0;
pub const FSL_QE_TSA_UCC1: c_int = 1;
pub const FSL_QE_TSA_UCC2: c_int = 2;
pub const FSL_QE_TSA_UCC3: c_int = 3;
pub const FSL_QE_TSA_UCC4: c_int = 4;
pub const FSL_QE_TSA_UCC5: c_int = 5;
