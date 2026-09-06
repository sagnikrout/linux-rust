//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_qm.h
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
// Number of bmaps that we ask from bmapi when doing a quotacheck.
// We make this restriction to keep the memory usage to a minimum.
//
pub const XFS_DQITER_MAP_SIZE: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_quota_limits {
    pub /: *mut *mut xfs_qcnt_t hard; / default hard limit,
    pub /: *mut *mut xfs_qcnt_t soft; / default soft limit,
    pub /: *mut *mut time64_t time; / limit for timers,
}

// Defaults for each quota type: time limits, warn limits, usage limits
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_def_quota {
    pub blk: xfs_quota_limits,
    pub ino: xfs_quota_limits,
    pub rtb: xfs_quota_limits,
}

//
// Various quota information for individual filesystems.
// The mount structure keeps a pointer to this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_quotainfo {
    pub qi_uquota_tree: radix_tree_root,
    pub qi_gquota_tree: radix_tree_root,
    pub qi_pquota_tree: radix_tree_root,
    pub qi_tree_lock: mutex,
    pub /: *mut *mut *mut xfs_inode qi_uquotaip; / user quota inode,
    pub /: *mut *mut *mut xfs_inode qi_gquotaip; / group quota inode,
    pub /: *mut *mut *mut xfs_inode qi_pquotaip; / project quota inode,
    pub /: *mut *mut *mut xfs_inode qi_dirip; / quota metadir,
    pub qi_lru: list_lru,
    pub qi_dquots: u64,
    pub /: *mut *mut mutex qi_quotaofflock;/ to serialize quotaoff,
    pub /: *mut *mut xfs_filblks_t qi_dqchunklen; / # BBs in a chunk of dqs,
    pub /: *mut *mut uint qi_dqperchunk; / # ondisk dq in above chunk,
    pub qi_usr_default: xfs_def_quota,
    pub qi_grp_default: xfs_def_quota,
    pub qi_prj_default: xfs_def_quota,
    pub qi_shrinker: *mut shrinker,
// Minimum and maximum quota expiration timestamp values.
    pub qi_expiry_min: time64_t,
    pub qi_expiry_max: time64_t,
// Hook to feed quota counter updates to an active online repair.
    pub qi_mod_ino_dqtrx_hooks: xfs_hooks,
    pub qi_apply_dqtrx_hooks: xfs_hooks,
}

//
// Parameters for tracking dqtrx changes on behalf of an inode.  The hook
// function arg parameter is the field being updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_mod_ino_dqtrx_params {
    pub tx_id: uintptr_t,
    pub ino: xfs_ino_t,
    pub q_type: xfs_dqtype_t,
    pub q_id: xfs_dqid_t,
    pub delta: i64,
}

extern "C" {
    pub fn xfs_trans_dqjoin(: *mut xfs_trans, : *mut xfs_dquot);
}
extern "C" {
    pub fn xfs_trans_log_dquot(: *mut xfs_trans, : *mut xfs_dquot);
}
//
// We keep the usr, grp, and prj dquots separately so that locking will be
// easier to do at commit time. All transactions that we know of at this point
// affect no more than two dquots of one type. Hence, the TRANS_MAXDQS value.
//
pub const XFS_QM_TRANS_MAXDQS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dquot_acct {
    pub dqs: [xfs_dqtrx; XFS_QM_TRANS_DQTYPES][XFS_QM_TRANS_MAXDQS],
}

//
// Users are allowed to have a usage exceeding their softlimit for
// a period this long.
//

extern "C" {
    pub fn xfs_qm_destroy_quotainfo(: *mut xfs_mount);
}
// quota ops
extern "C" {
    pub fn xfs_qm_scall_trunc_qfiles(: *mut xfs_mount, _arg: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_qm_scall_quotaon(: *mut xfs_mount, _arg: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_qm_scall_quotaoff(: *mut xfs_mount, _arg: c_uint) -> c_int;
}
