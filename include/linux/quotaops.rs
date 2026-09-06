//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/quotaops.h
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
// Definitions for diskquota-operations. When diskquota is configured these
// macros expand to the right source-code.
//
// Author:  Marco van Wieringen <mvw@planets.elm.net>
//

pub const DQUOT_SPACE_WARN: c_uint = 0x1;
pub const DQUOT_SPACE_RESERVE: c_uint = 0x2;
pub const DQUOT_SPACE_NOFAIL: c_uint = 0x4;
// i_rwsem must being held

//
// declaration of quota_function calls in kernel.
//
extern "C" {
    pub fn dquot_initialize(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn dquot_initialize_needed(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn dquot_drop(inode: *mut inode);
}
extern "C" {
    pub fn dqput(dquot: *mut dquot);
}
extern "C" {
    pub fn dquot_destroy(dquot: *mut dquot);
}
extern "C" {
    pub fn __dquot_alloc_space(inode: *mut inode, number: qsize_t, flags: c_int) -> c_int;
}
extern "C" {
    pub fn __dquot_free_space(inode: *mut inode, number: qsize_t, flags: c_int);
}
extern "C" {
    pub fn dquot_alloc_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn dquot_claim_space_nodirty(inode: *mut inode, number: qsize_t);
}
extern "C" {
    pub fn dquot_free_inode(inode: *mut inode);
}
extern "C" {
    pub fn dquot_reclaim_space_nodirty(inode: *mut inode, number: qsize_t);
}
extern "C" {
    pub fn dquot_disable(sb: *mut super_block, type: c_int, flags: c_uint) -> c_int;
}
// Suspend quotas on remount RO
extern "C" {
    pub fn dquot_disable(_arg: sb, _arg: type, _arg: DQUOT_SUSPENDED) -> return;
}
extern "C" {
    pub fn dquot_resume(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn dquot_commit(dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn dquot_acquire(dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn dquot_release(dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn dquot_commit_info(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn dquot_get_next_id(sb: *mut super_block, qid: *mut kqid) -> c_int;
}
extern "C" {
    pub fn dquot_mark_dquot_dirty(dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn dquot_file_open(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn dquot_quota_off(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn dquot_writeback_dquots(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn dquot_quota_sync(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn dquot_get_state(sb: *mut super_block, state: *mut qc_state) -> c_int;
}
extern "C" {
    pub fn dquot_set_dqinfo(sb: *mut super_block, type: c_int, ii: *mut qc_info) -> c_int;
}
extern "C" {
    pub fn __dquot_transfer(inode: *mut inode, transfer_to: *mut dquot) -> c_int;
}
//
// Functions for checking status of quota
//
extern "C" {
    pub fn dquot_state_types(_arg: sb_dqopt(sb)->flags, _arg: DQUOT_SUSPENDED) -> return;
}
// Does kernel know about any quota information for given sb + type?
// Currently if anything is on, then quota usage is on as well
extern "C" {
    pub fn sb_has_quota_usage_enabled(_arg: sb, _arg: type) -> return;
}
extern "C" {
    pub fn dquot_state_types(_arg: sb_dqopt(sb)->flags, _arg: DQUOT_USAGE_ENABLED) -> return;
}
//
// Operations supported for diskquotas.
//

// Does kernel know about any quota information for given sb + type?

extern "C" {
    pub fn __dquot_alloc_space(_arg: inode, _arg: nr, _arg: DQUOT_SPACE_WARN) -> return;
}
//
// Mark inode fully dirty. Since we are allocating blocks, inode
// would become fully dirty soon anyway and it reportedly
// reduces lock contention.
//
extern "C" {
    pub fn dquot_alloc_space_nodirty(_arg: inode, inode->i_blkbits: nr <<) -> return;
}
extern "C" {
    pub fn dquot_alloc_space(_arg: inode, inode->i_blkbits: nr <<) -> return;
}
extern "C" {
    pub fn __dquot_alloc_space(_arg: inode, inode->i_blkbits: nr <<, _arg: 0) -> return;
}
extern "C" {
    pub fn qtype_enforce_flag(type: c_int) -> c_uint;
}
