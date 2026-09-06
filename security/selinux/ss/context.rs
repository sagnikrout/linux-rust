//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/context.h
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
// A security context is a set of security attributes
// associated with each subject and object controlled
// by the security policy.  Security contexts are
// externally represented as variable-length strings
// that can be interpreted by a user or application
// with an understanding of the security policy.
// Internally, the security server uses a simple
// structure.  This structure is private to the
// security server and can be changed without affecting
// clients of the security server.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

//
// A security context consists of an authenticated user
// identity, a role, a type and a MLS range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct context {
    pub user: u32,
    pub role: u32,
    pub type: u32,
    pub /: *mut *mut u32 len; / length of string in bytes,
    pub range: mls_range,
    pub /: *mut *mut *mut char str; / string representation if context cannot be mapped.,
}

//
// Sets both levels in the MLS range of 'dst' to the low level of 'src'.
//
// Sets both levels in the MLS range of 'dst' to the high level of 'src'.
//
// These ranges have no common sensitivities
// Take the greatest of the low
// Take the least of the high
extern "C" {
    pub fn context_compute_hash(c: *const context) -> u32;
}
