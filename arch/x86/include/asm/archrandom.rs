//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/archrandom.h
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
// This file is part of the Linux kernel.
//
// Copyright (c) 2011-2014, Intel Corporation
// Authors: Fenghua Yu <fenghua.yu@intel.com>,
// H. Peter Anvin <hpa@linux.intel.com>
//

pub const RDRAND_RETRY_LOOPS: c_int = 10;
// Unconditional execution of RDRAND and RDSEED
//
// These are the generic interfaces; they must not be declared if the
// stubs in <linux/random.h> are to be invoked.
//

extern "C" {
    pub fn x86_init_rdrand(c: *mut cpuinfo_x86);
}

