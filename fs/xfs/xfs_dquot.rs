//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_dquot.h
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
// Dquots are structures that hold quota information about a user or a group,
// much like inodes are for files. In fact, dquots share many characteristics
// with inodes. However, dquots can also be a centralized resource, relative
// to a collection of inodes. In this respect, dquots share some characteristics
// of the superblock.
// XFS dquots exploit both those in its algorithms. They make every attempt
// to not be a bottleneck when quotas are on and have minimal impact, if any,
// when quotas are off.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dquot_res {
// Total resources allocated and reserved.
    pub reserved: xfs_qcnt_t,
// Total resources allocated.
    pub count: xfs_qcnt_t,
// Absolute and preferred limits.
    pub hardlimit: xfs_qcnt_t,
    pub softlimit: xfs_qcnt_t,
//
// For root dquots, this is the default grace period, in seconds.
// Otherwise, this is when the quota grace period expires,
// in seconds since the Unix epoch.
//
    pub timer: time64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dquot_pre {
    pub q_prealloc_lo_wmark: xfs_qcnt_t,
    pub q_prealloc_hi_wmark: xfs_qcnt_t,
    pub q_low_space: [i64; XFS_QLOWSP_MAX],
}

//
// The incore dquot structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dquot {
    pub q_lru: list_head,
    pub q_mount: *mut xfs_mount,
    pub q_type: xfs_dqtype_t,
    pub q_flags: u16,
    pub q_id: xfs_dqid_t,
    pub q_lockref: lockref,
    pub q_bufoffset: c_int,
    pub q_blkno: xfs_daddr_t,
    pub q_fileoffset: xfs_fileoff_t,
    pub /: *mut *mut xfs_dquot_res q_blk; / regular blocks,
    pub /: *mut *mut xfs_dquot_res q_ino; / inodes,
    pub /: *mut *mut xfs_dquot_res q_rtb; / realtime blocks,
    pub q_logitem: xfs_dq_logitem,
    pub q_blk_prealloc: xfs_dquot_pre,
    pub q_rtb_prealloc: xfs_dquot_pre,
    pub q_qlock: mutex,
    pub q_flush: completion,
    pub q_pincount: core::sync::atomic::AtomicI32,
    pub q_pinwait: wait_queue_head,
}

//
// Lock hierarchy for q_qlock:
// XFS_QLOCK_NORMAL is the implicit default,
// XFS_QLOCK_NESTED is the dquot with the higher id in xfs_dqlock2
//
// Manage the q_flush completion queue embedded in the dquot. This completion
// queue synchronizes processes attempting to flush the in-core dquot back to
// disk.
//
extern "C" {
    pub fn try_wait_for_completion(_arg: &dqp->q_flush) -> return;
}
extern "C" {
    pub fn XFS_IS_UQUOTA_ON(_arg: mp) -> return;
}
extern "C" {
    pub fn XFS_IS_GQUOTA_ON(_arg: mp) -> return;
}
extern "C" {
    pub fn XFS_IS_PQUOTA_ON(_arg: mp) -> return;
}
// Decide if the dquot's limits are actually being enforced.
extern "C" {
    pub fn XFS_IS_UQUOTA_ENFORCED(_arg: dqp->q_mount) -> return;
}
extern "C" {
    pub fn XFS_IS_GQUOTA_ENFORCED(_arg: dqp->q_mount) -> return;
}
extern "C" {
    pub fn XFS_IS_PQUOTA_ENFORCED(_arg: dqp->q_mount) -> return;
}
//
// Check whether a dquot is under low free space conditions. We assume the quota
// is enabled and enforced.
//
extern "C" {
    pub fn xfs_dquot_to_disk(ddqp: *mut xfs_disk_dquot, dqp: *mut xfs_dquot);
}

extern "C" {
    pub fn xfs_qm_dqdestroy(dqp: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_qm_dqflush(dqp: *mut xfs_dquot, bp: *mut xfs_buf) -> c_int;
}
extern "C" {
    pub fn xfs_qm_dqunpin_wait(dqp: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_qm_adjust_dqtimers(d: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_qm_adjust_dqlimits(d: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_dqlock2(: *mut xfs_dquot, : *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_dqlockn(q: *mut xfs_dqtrx);
}
extern "C" {
    pub fn xfs_dquot_set_prealloc_limits(: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_dquot_attach_buf(tp: *mut xfs_trans, dqp: *mut xfs_dquot) -> c_int;
}
extern "C" {
    pub fn xfs_dquot_use_attached_buf(dqp: *mut xfs_dquot, bpp: *mut xfs_buf) -> c_int;
}
extern "C" {
    pub fn xfs_dquot_detach_buf(dqp: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_dquot_set_timeout(mp: *mut xfs_mount, timeout: time64_t) -> time64_t;
}
extern "C" {
    pub fn xfs_dquot_set_grace_period(grace: time64_t) -> time64_t;
}
