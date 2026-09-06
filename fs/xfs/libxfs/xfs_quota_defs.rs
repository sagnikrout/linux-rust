//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_quota_defs.h
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Quota definitions shared between user and kernel source trees.
//
// Even though users may not have quota limits occupying all 64-bits,
// they may need 64-bit accounting. Hence, 64-bit quota-counters,
// and quota-limits. This is a waste in the common case, but hey ...
//
pub type xfs_qcnt_t = u64;
pub type xfs_dqtype_t = u8;

//
// flags for q_flags field in the dquot.
//

//
// We have the possibility of all three quota types being active at once, and
// hence free space modification requires modification of all three current
// dquots in a single transaction. For this case we need to have a reservation
// of at least 3 dquots.
//
// However, a chmod operation can change both UID and GID in a single
// transaction, resulting in requiring {old, new} x {uid, gid} dquots to be
// modified. Hence for this case we need to reserve space for at least 4 dquots.
//
// And in the worst case, there's a rename operation that can be modifying up to
// 4 inodes with dquots attached to them. In reality, the only inodes that can
// have their dquots modified are the source and destination directory inodes
// due to directory name creation and removal. That can require space allocation
// and/or freeing on both directory inodes, and hence all three dquots on each
// inode can be modified. And if the directories are world writeable, all the
// dquots can be unique and so 6 dquots can be modified....
//
// And, of course, we also need to take into account the dquot log format item
// used to describe each dquot.
//

//
// Flags to tell various functions what to do. Not all of these are meaningful
// to a single function. None of these XFS_QMOPT_* flags are meant to have
// persistent values (ie. their values can and will change between versions)
//

//
// flags to xfs_trans_mod_dquot to indicate which field needs to be
// modified.
//

//
// flags for dqalloc.
//

//
// flags to xfs_trans_mod_dquot.
//

extern "C" {
    pub fn xfs_calc_dquots_per_chunk(nbblks: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_dquot_to_disk_ts(ddq: *mut xfs_dquot, timer: time64_t) -> __be32;
}
extern "C" {
    pub fn xfs_dqinode_sick_mask(type: xfs_dqtype_t) -> c_uint;
}
extern "C" {
    pub fn xfs_dqinode_mkdir_parent(mp: *mut xfs_mount, dpp: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_dqinode_load_parent(tp: *mut xfs_trans, dpp: *mut xfs_inode) -> c_int;
}
