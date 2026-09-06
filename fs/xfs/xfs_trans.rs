//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_trans.h
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
// Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// kernel only transaction subsystem defines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_log_item {
    pub /: *mut *mut list_head li_ail; / AIL pointers,
    pub /: *mut *mut list_head li_trans; / transaction list,
    pub /: *mut *mut xfs_lsn_t li_lsn; / last on-disk lsn,
    pub li_log: *mut xlog,
    pub /: *mut *mut *mut xfs_ail li_ailp; / ptr to AIL,
    pub /: *mut *mut uint li_type; / item type,
    pub /: *mut *mut unsigned long li_flags; / misc flags,
    pub /: *mut *mut *mut xfs_buf li_buf; / real buffer pointer,
    pub /: *mut *mut list_head li_bio_list; / buffer item list,
    pub /: *const *const *const xfs_item_ops li_ops; / function list,
// delayed logging
    pub /: *mut *mut list_head li_cil; / CIL pointers,
    pub /: *mut *mut *mut xfs_log_vec li_lv; / active log vector,
    pub /: *mut *mut *mut xfs_log_vec li_lv_shadow; / standby vector,
    pub /: *mut *mut xfs_csn_t li_seq; / CIL commit seq,
    pub /: *mut *mut uint32_t li_order_id; / CIL commit order,
}

//
// li_flags use the (set/test/clear)_bit atomic interfaces because updates can
// race with each other and we don't want to have to use the AIL lock to
// serialise all updates.
//
pub const XFS_LI_IN_AIL: c_int = 0;
pub const XFS_LI_ABORTED: c_int = 1;
pub const XFS_LI_FAILED: c_int = 2;
pub const XFS_LI_DIRTY: c_int = 3;
pub const XFS_LI_WHITEOUT: c_int = 4;
pub const XFS_LI_FLUSHING: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_item_ops {
    pub flags: unsigned,
    pub ): *mut *mut *mut *mut void (iop_size)(struct xfs_log_item , int , int,
    pub lfb): *mut xlog_format_buf,
    pub ): *mut *mut void (iop_pin)(struct xfs_log_item,
    pub remove): *mut *mut *mut void (iop_unpin)(struct xfs_log_item , int,
    pub lip): *mut *mut uint64_t (iop_sort)(struct xfs_log_item,
    pub lip): *mut *mut *mut int (iop_precommit)(struct xfs_trans tp, struct xfs_log_item,
    pub seq): *mut *mut *mut void (iop_committing)(struct xfs_log_item lip, xfs_csn_t,
    pub xfs_lsn_t): *mut *mut *mut xfs_lsn_t (iop_committed)(struct xfs_log_item ,,
    pub ): *mut *mut *mut uint (iop_push)(struct xfs_log_item , struct list_head,
    pub ): *mut *mut void (iop_release)(struct xfs_log_item,
    pub id): *mut *mut *mut bool (iop_match)(struct xfs_log_item item, uint64_t,
    pub intent_done): *mut *mut *mut xfs_log_item (iop_intent)(xfs_log_item,
}

//
// Log item ops flags
//
// Release the log item when the journal commits instead of inserting into the
// AIL for writeback tracking and/or log tail pinning.
//

//
// Return values for the iop_push() routines.
//
pub const XFS_ITEM_SUCCESS: c_int = 0;
pub const XFS_ITEM_PINNED: c_int = 1;
pub const XFS_ITEM_LOCKED: c_int = 2;
pub const XFS_ITEM_FLUSHING: c_int = 3;
//
// This is the structure maintained for every active transaction.
//
// XFS transaction mechanism exported interfaces that are
// actually macros.
//

//
// XFS transaction mechanism exported interfaces.
//
extern "C" {
    pub fn xfs_trans_mod_sb(: *mut xfs_trans_t, _arg: c_uint, _arg: i64);
}
extern "C" {
    pub fn xfs_trans_get_buf_map(_arg: tp, _arg: target, _arg: &map, _arg: 1, _arg: flags, _arg: bpp) -> return;
}
extern "C" {
    pub fn xfs_trans_brelse(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_bjoin(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_bdetach(tp: *mut xfs_trans, bp: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_bhold(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_bhold_release(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_binval(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_inode_buf(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_stale_inode_buf(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_ordered_buf(: *mut xfs_trans_t, : *mut xfs_buf) -> bool;
}
extern "C" {
    pub fn xfs_trans_dquot_buf(: *mut xfs_trans_t, : *mut xfs_buf, _arg: c_uint);
}
extern "C" {
    pub fn xfs_trans_inode_alloc_buf(: *mut xfs_trans_t, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_ijoin(: *mut xfs_trans, : *mut xfs_inode, _arg: c_uint);
}
extern "C" {
    pub fn xfs_trans_dirty_buf(: *mut xfs_trans, : *mut xfs_buf);
}
extern "C" {
    pub fn xfs_trans_buf_is_dirty(bp: *mut xfs_buf) -> bool;
}
extern "C" {
    pub fn xfs_trans_log_inode(: *mut xfs_trans_t, : *mut xfs_inode, _arg: c_uint);
}
extern "C" {
    pub fn xfs_trans_commit(: *mut xfs_trans) -> c_int;
}
extern "C" {
    pub fn xfs_trans_roll(: *mut xfs_trans) -> c_int;
}
extern "C" {
    pub fn xfs_trans_roll_inode(: *mut xfs_trans, : *mut xfs_inode) -> c_int;
}
extern "C" {
    pub fn xfs_trans_cancel(: *mut xfs_trans_t);
}
extern "C" {
    pub fn xfs_trans_ail_init(: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_trans_ail_destroy(: *mut xfs_mount);
}
