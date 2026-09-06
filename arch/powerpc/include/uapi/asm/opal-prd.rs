//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/opal-prd.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// OPAL Runtime Diagnostics interface driver
// Supported on POWERNV platform
//
// (C) Copyright IBM 2015
//
// Author: Vaidyanathan Srinivasan <svaidy at linux.vnet.ibm.com>
// Author: Jeremy Kerr <jk@ozlabs.org>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

//
// The version of the kernel interface of the PRD system. This describes the
// interface available for the /dev/opal-prd device. The actual PRD message
// layout and content is private to the firmware <--> userspace interface, so
// is not covered by this versioning.
//
// Future interface versions are backwards-compatible; if a later kernel
// version is encountered, functionality provided in earlier versions
// will work.
//
pub const OPAL_PRD_KERNEL_VERSION: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_prd_info {
    pub version: __u64,
    pub reserved: [__u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_prd_scom {
    pub chip: __u64,
    pub addr: __u64,
    pub data: __u64,
    pub rc: __s64,
}

