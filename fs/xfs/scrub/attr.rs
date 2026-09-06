//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/attr.h
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
// Copyright (C) 2019-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// Temporary storage for online scrub and repair of extended attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_xattr_buf {
// Bitmap of used space in xattr leaf blocks and shortform forks.
    pub usedmap: *mut c_ulong,
// Bitmap of free space in xattr leaf blocks.
    pub freemap: *mut c_ulong,
// Memory buffer used to hold salvaged xattr names.
    pub name: *mut c_uchar,
// Memory buffer used to extract xattr values.
    pub value: *mut c_void,
    pub value_sz: usize,
}

extern "C" {
    pub fn xchk_setup_xattr_buf(sc: *mut xfs_scrub, value_size: usize) -> c_int;
}
