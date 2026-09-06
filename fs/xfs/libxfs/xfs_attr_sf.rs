//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_attr_sf.h
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
// Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// We generate this then sort it, attr_list() must return things in hash-order.
//

// space name/value uses
// space an entry uses
extern "C" {
    pub fn struct_size(_arg: sfep, _arg: nameval, sfep->valuelen: sfep->namelen +) -> return;
}
// first entry in the SF attr fork
// next entry after sfep
// pointer to the space after the last entry, e.g. for adding a new one
