//! Automatically rewritten from C Header to Rust Module
//! Source: security/keys/encrypted-keys/ecryptfs_format.h
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
// ecryptfs_format.h: helper functions for the encrypted key type
//
// Copyright (C) 2006 International Business Machines Corp.
// Copyright (C) 2010 Politecnico di Torino, Italy
// TORSEC group -- https://security.polito.it
//
// Authors:
// Michael A. Halcrow <mahalcro@us.ibm.com>
// Tyler Hicks <tyhicks@ou.edu>
// Roberto Sassu <roberto.sassu@polito.it>
//

pub const PGP_DIGEST_ALGO_SHA512: c_int = 10;
extern "C" {
    pub fn ecryptfs_get_versions(major: *mut c_int, minor: *mut c_int, file_version: *mut c_int);
}
