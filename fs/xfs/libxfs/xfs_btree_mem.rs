//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_btree_mem.h
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
// Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
pub type xfbno_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfbtree {
// buffer cache target for this in-memory btree
    pub target: *mut xfs_buftarg,
// Highest block number that has been written to.
    pub highest_bno: xfbno_t,
// Owner of this btree.
    pub owner: c_ulonglong,
// Btree header
    pub root: xfs_btree_ptr,
    pub nlevels: c_uint,
// Minimum and maximum records per block.
    pub maxrecs: [c_uint; 2],
    pub minrecs: [c_uint; 2],
}

extern "C" {
    pub fn xmbuf_verify_daddr(_arg: xfbt->target, _arg: xfbno_to_daddr(bno)) -> return;
}
extern "C" {
    pub fn xfbtree_get_minrecs(cur: *mut xfs_btree_cur, level: c_int) -> c_int;
}
extern "C" {
    pub fn xfbtree_get_maxrecs(cur: *mut xfs_btree_cur, level: c_int) -> c_int;
}
extern "C" {
    pub fn xfbtree_free_block(cur: *mut xfs_btree_cur, bp: *mut xfs_buf) -> c_int;
}
// Callers must set xfbt->target and xfbt->owner before calling this
extern "C" {
    pub fn xfbtree_destroy(xfbt: *mut xfbtree);
}
extern "C" {
    pub fn xfbtree_trans_commit(xfbt: *mut xfbtree, tp: *mut xfs_trans) -> c_int;
}
extern "C" {
    pub fn xfbtree_trans_cancel(xfbt: *mut xfbtree, tp: *mut xfs_trans);
}

