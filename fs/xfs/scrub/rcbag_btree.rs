//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/rcbag_btree.h
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
// Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

pub const RCBAG_MAGIC: c_uint = 0x74826671	/* 'JRBG' */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcbag_key {
    pub rbg_startblock: u32,
    pub rbg_blockcount: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcbag_rec {
    pub rbg_startblock: u32,
    pub rbg_blockcount: u32,
    pub rbg_refcount: u64,
}

pub type rcbag_ptr_t = __be64;
// reflinks only exist on crc enabled filesystems

//
// Record, key, and pointer address macros for btree blocks.
//
// (note that some of these may appear unused, but they are used in userspace)
//

extern "C" {
    pub fn rcbagbt_calc_size(nr_records: c_ulonglong) -> c_ulonglong;
}
extern "C" {
    pub fn rcbagbt_maxlevels_possible() -> c_uint;
}
extern "C" {
    pub fn rcbagbt_init_cur_cache() -> int __init;
}
extern "C" {
    pub fn rcbagbt_destroy_cur_cache();
}
extern "C" {
    pub fn rcbagbt_get_rec(cur: *mut xfs_btree_cur, rec: *mut rcbag_rec, has: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rcbagbt_update(cur: *mut xfs_btree_cur, rec: *const rcbag_rec) -> c_int;
}

