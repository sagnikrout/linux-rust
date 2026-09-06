//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_quota.h
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

//
// Kernel only quota definitions and functions
//
// This check is done typically without holding the inode lock;
// that may seem racy, but it is harmless in the context that it is used.
// The inode cannot go inactive as long a reference is kept, and
// therefore if dquot(s) were attached, they'll stay consistent.
// If, for example, the ownership of the inode changes while
// we didn't have the inode locked, the appropriate dquot(s) will be
// attached atomically.
//

//
// The structure kept inside the xfs_trans_t keep track of dquot changes
// within a transaction and apply them later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dqtrx {
    pub /: *mut *mut *mut xfs_dquot qt_dquot; / the dquot this refers to,
    pub /: *mut *mut uint64_t qt_blk_res; / blks reserved on a dquot,
    pub /: *mut *mut int64_t qt_bcount_delta; / dquot blk count changes,
    pub /: *mut *mut int64_t qt_delbcnt_delta; / delayed dquot blk count changes,
    pub /: *mut *mut uint64_t qt_rtblk_res; / # blks reserved on a dquot,
    pub /: *mut *mut uint64_t qt_rtblk_res_used;/ # blks used from reservation,
    pub /: *mut *mut int64_t qt_rtbcount_delta;/ dquot realtime blk changes,
    pub /: *mut *mut int64_t qt_delrtb_delta; / delayed RT blk count changes,
    pub /: *mut *mut uint64_t qt_ino_res; / inode reserved on a dquot,
    pub /: *mut *mut uint64_t qt_ino_res_used; / inodes used from the reservation,
    pub /: *mut *mut int64_t qt_icount_delta; / dquot inode count changes,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_apply_dqtrx_type {
    XFS_APPLY_DQTRX_COMMIT = 0,
    XFS_APPLY_DQTRX_UNRESERVE,
}

//
// Parameters for applying dqtrx changes to a dquot.  The hook function arg
// parameter is enum xfs_apply_dqtrx_type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_apply_dqtrx_params {
    pub tx_id: uintptr_t,
    pub ino: xfs_ino_t,
    pub q_type: xfs_dqtype_t,
    pub q_id: xfs_dqid_t,
}

extern "C" {
    pub fn xfs_trans_dup_dqinfo(: *mut xfs_trans, : *mut xfs_trans);
}
extern "C" {
    pub fn xfs_trans_free_dqinfo(: *mut xfs_trans);
}
extern "C" {
    pub fn xfs_trans_apply_dquot_deltas(: *mut xfs_trans);
}
extern "C" {
    pub fn xfs_qm_vop_rename_dqattach(: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_qm_dqattach(: *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_qm_dqattach_locked(ip: *mut xfs_inode, doalloc: bool) -> c_int;
}
extern "C" {
    pub fn xfs_qm_dqdetach(: *mut xfs_inode);
}
extern "C" {
    pub fn xfs_qm_dqrele(: *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_qm_statvfs(: *mut xfs_inode, : *mut kstatfs);
}
extern "C" {
    pub fn xfs_qm_newmount(: *mut xfs_mount, : *mut c_uint, : *mut c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_qm_resume_quotaon(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_qm_mount_quotas(: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_qm_unmount(: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_qm_unmount_quotas(: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_inode_near_dquot_enforcement(ip: *mut xfs_inode, type: xfs_dqtype_t) -> bool;
}
extern "C" {
    pub fn xfs_quota_reserve_blkres(ip: *mut xfs_inode, blocks: i64) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dqtrx_hook {
    pub mod_hook: xfs_hook,
    pub apply_hook: xfs_hook,
}

extern "C" {
    pub fn xfs_dqtrx_hook_disable();
}
extern "C" {
    pub fn xfs_dqtrx_hook_enable();
}
extern "C" {
    pub fn xfs_dqtrx_hook_add(qi: *mut xfs_quotainfo, hook: *mut xfs_dqtrx_hook) -> c_int;
}
extern "C" {
    pub fn xfs_dqtrx_hook_del(qi: *mut xfs_quotainfo, hook: *mut xfs_dqtrx_hook);
}

// udqp = NULL;
// gdqp = NULL;
// pdqp = NULL;

// Macro flag: #define xfs_trans_free_dqinfo(tp)
// Macro flag: #define xfs_trans_apply_dquot_deltas(tp)

// Macro flag: #define xfs_qm_dqdetach(ip)

// Macro flag: #define xfs_qm_mount_quotas(mp)
// Macro flag: #define xfs_qm_unmount(mp)
// Macro flag: #define xfs_qm_unmount_quotas(mp)

// don't return an error as unreserving quotas can't fail
extern "C" {
    pub fn xfs_mount_reset_sbqflags(: *mut xfs_mount) -> c_int;
}
