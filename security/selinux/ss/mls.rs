//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/mls.h
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
// Multi-level security (MLS) policy operations.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//
// Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
// Support for enhanced MLS infrastructure.
// Copyright (C) 2004-2006 Trusted Computer Solutions, Inc.
//
// Updated: Hewlett-Packard <paul@paul-moore.com>
// Added support to import/export the MLS label from NetLabel
// Copyright (X) Hewlett-Packard Development Company, L.P., 2006
//

extern "C" {
    pub fn mls_compute_context_len(p: *mut policydb, context: *mut context) -> c_int;
}
extern "C" {
    pub fn mls_context_isvalid(p: *const policydb, c: *const context) -> bool;
}
extern "C" {
    pub fn mls_range_isvalid(p: *const policydb, r: *const mls_range) -> bool;
}
extern "C" {
    pub fn mls_level_isvalid(p: *const policydb, l: *const mls_level) -> bool;
}
extern "C" {
    pub fn mls_range_set(context: *mut context, range: *mut mls_range) -> c_int;
}

