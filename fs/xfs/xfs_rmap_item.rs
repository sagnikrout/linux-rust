//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_rmap_item.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//
// There are (currently) three pairs of rmap btree redo item types: map, unmap,
// and convert.  The common abbreviations for these are RUI (rmap update
// intent) and RUD (rmap update done).  The redo item type is encoded in the
// flags field of each xfs_map_extent.
//
// *I items should be recorded in the *first* of a series of rolled
// transactions, and the *D items should be recorded in the same transaction
// that records the associated rmapbt updates.  Typically, the first
// transaction will record a bmbt update, followed by some number of
// transactions containing rmapbt updates, and finally transactions with any
// bnobt/cntbt updates.
//
// Should the system crash after the commit of the first transaction but
// before the commit of the final transaction in a series, log recovery will
// use the redo information recorded by the intent items to replay the
// (rmapbt/bnobt/cntbt) metadata updates in the non-first transaction.
//
// kernel only RUI/RUD definitions
//
// Max number of extents in fast allocation path.
//
pub const XFS_RUI_MAX_FAST_EXTENTS: c_int = 16;
//
// This is the "rmap update intent" log item.  It is used to log the fact that
// some reverse mappings need to change.  It is used in conjunction with the
// "rmap update done" log item described below.
//
// These log items follow the same rules as struct xfs_efi_log_item; see the
// comments about that structure (in xfs_extfree_item.h) for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rui_log_item {
    pub rui_item: xfs_log_item,
    pub rui_refcount: core::sync::atomic::AtomicI32,
    pub rui_next_extent: core::sync::atomic::AtomicI32,
    pub rui_format: xfs_rui_log_format,
}

//
// This is the "rmap update done" log item.  It is used to log the fact that
// some rmapbt updates mentioned in an earlier rui item have been performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_rud_log_item {
    pub rud_item: xfs_log_item,
    pub rud_ruip: *mut xfs_rui_log_item,
    pub rud_format: xfs_rud_log_format,
}

extern "C" {
    pub fn xfs_rmap_defer_add(tp: *mut xfs_trans, ri: *mut xfs_rmap_intent);
}
extern "C" {
    pub fn xfs_rui_log_space(nr: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_rud_log_space() -> c_uint;
}
