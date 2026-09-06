//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/runlist.h
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
// Defines for runlist handling in NTFS Linux kernel driver.
//
// Copyright (c) 2001-2005 Anton Altaparmakov
// Copyright (c) 2002 Richard Russon
// Copyright (c) 2025 LG Electronics Co., Ltd.
//

//
// runlist_element - in memory vcn to lcn mapping array element
// @vcn:	starting vcn of the current array element
// @lcn:	starting lcn of the current array element
// @length:	length in clusters of the current array element
//
// The last vcn (in fact the last vcn + 1) is reached when length == 0.
//
// When lcn == -1 this means that the count vcns starting at vcn are not
// physically allocated (i.e. this is a hole / data is sparse).
//
// In memory vcn to lcn mapping structure element.
// @vcn: vcn = Starting virtual cluster number.
// @lcn: lcn = Starting logical cluster number.
// @length: Run length in clusters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct runlist_element {
    pub vcn: i64,
    pub lcn: i64,
    pub length: i64,
}

//
// runlist - in memory vcn to lcn mapping array including a read/write lock
// @rl:		pointer to an array of runlist elements
// @lock:	read/write spinlock for serializing access to @rl
// @rl_hint:	hint/cache pointing to the last accessed runlist element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct runlist {
    pub rl: *mut runlist_element,
    pub lock: rw_semaphore,
    pub count: usize,
    pub rl_hint: c_int,
}

extern "C" {
    pub fn ntfs_rl_vcn_to_lcn(rl: *const runlist_element, vcn: i64) -> i64;
}
extern "C" {
    pub fn ntfs_rl_sparse(rl: *mut runlist_element) -> c_int;
}
extern "C" {
    pub fn ntfs_rl_get_compressed_size(vol: *mut ntfs_volume, rl: *mut runlist_element) -> i64;
}
