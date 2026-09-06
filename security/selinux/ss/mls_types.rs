//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/mls_types.h
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
// Type definitions for the multi-level security (MLS) policy.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//
// Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
// Support for enhanced MLS infrastructure.
// Copyright (C) 2004-2005 Trusted Computer Solutions, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mls_level {
    pub /: *mut *mut u32 sens; / sensitivity,
    pub /: *mut *mut ebitmap cat; / category set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mls_range {
    pub /: *mut *mut mls_level level[2]; / low == level[0], high == level[1],
}

