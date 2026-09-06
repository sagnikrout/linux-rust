//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace-vsx.h
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
// Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
//
pub const VEC_MAX: c_int = 128;
pub const VSX_MAX: c_int = 32;
pub const VMX_MAX: c_int = 32;
//
// unsigned long vsx[32]
// unsigned long load[128]
//
// unsigned long vmx[32][2]
// unsigned long load[128]
//

// In LE each value pair is stored in an
// alternate manner.
//

//
// unsigned long store[128]
// unsigned long load[128]
//

extern "C" {
    pub fn loadvsx(p: *mut c_void, tmp: c_int);
}
extern "C" {
    pub fn storevsx(p: *mut c_void, tmp: c_int);
}
