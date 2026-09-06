//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_log.h
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
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// Region types for iovec's i_type
pub const XLOG_REG_TYPE_BFORMAT: c_int = 1;
pub const XLOG_REG_TYPE_BCHUNK: c_int = 2;
pub const XLOG_REG_TYPE_EFI_FORMAT: c_int = 3;
pub const XLOG_REG_TYPE_EFD_FORMAT: c_int = 4;
pub const XLOG_REG_TYPE_IFORMAT: c_int = 5;
pub const XLOG_REG_TYPE_ICORE: c_int = 6;
pub const XLOG_REG_TYPE_IEXT: c_int = 7;
pub const XLOG_REG_TYPE_IBROOT: c_int = 8;
pub const XLOG_REG_TYPE_ILOCAL: c_int = 9;
pub const XLOG_REG_TYPE_IATTR_EXT: c_int = 10;
pub const XLOG_REG_TYPE_IATTR_BROOT: c_int = 11;
pub const XLOG_REG_TYPE_IATTR_LOCAL: c_int = 12;
pub const XLOG_REG_TYPE_QFORMAT: c_int = 13;
pub const XLOG_REG_TYPE_DQUOT: c_int = 14;
pub const XLOG_REG_TYPE_QUOTAOFF: c_int = 15;
pub const XLOG_REG_TYPE_LRHEADER: c_int = 16;
pub const XLOG_REG_TYPE_UNMOUNT: c_int = 17;
pub const XLOG_REG_TYPE_COMMIT: c_int = 18;
pub const XLOG_REG_TYPE_TRANSHDR: c_int = 19;
pub const XLOG_REG_TYPE_ICREATE: c_int = 20;
pub const XLOG_REG_TYPE_RUI_FORMAT: c_int = 21;
pub const XLOG_REG_TYPE_RUD_FORMAT: c_int = 22;
pub const XLOG_REG_TYPE_CUI_FORMAT: c_int = 23;
pub const XLOG_REG_TYPE_CUD_FORMAT: c_int = 24;
pub const XLOG_REG_TYPE_BUI_FORMAT: c_int = 25;
pub const XLOG_REG_TYPE_BUD_FORMAT: c_int = 26;
pub const XLOG_REG_TYPE_ATTRI_FORMAT: c_int = 27;
pub const XLOG_REG_TYPE_ATTRD_FORMAT: c_int = 28;
pub const XLOG_REG_TYPE_ATTR_NAME: c_int = 29;
pub const XLOG_REG_TYPE_ATTR_VALUE: c_int = 30;
pub const XLOG_REG_TYPE_XMI_FORMAT: c_int = 31;
pub const XLOG_REG_TYPE_XMD_FORMAT: c_int = 32;
pub const XLOG_REG_TYPE_ATTR_NEWNAME: c_int = 33;
pub const XLOG_REG_TYPE_ATTR_NEWVALUE: c_int = 34;
pub const XLOG_REG_TYPE_MAX: c_int = 34;

//
// Calculate the log iovec length for a given user buffer length. Intended to be
// used by ->iop_size implementations when sizing buffers of arbitrary
// alignments.
//
extern "C" {
    pub fn roundup(_arg: len, _arg: sizeof(uint32_t)) -> return;
}
extern "C" {
    pub fn xlog_format_commit(lfb: *mut xlog_format_buf, data_len: c_uint);
}
//
// Copy the amount of data requested by the caller into a new log iovec.
//
// Flags to xfs_log_force()
//
// XFS_LOG_SYNC:	Synchronous force in-core log to disk
//
pub const XFS_LOG_SYNC: c_uint = 0x1;
// Log manager interfaces
extern "C" {
    pub fn xfs_log_force(mp: *mut xfs_mount, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn xfs_log_mount_finish(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_log_mount_cancel(: *mut xfs_mount);
}
extern "C" {
    pub fn xlog_assign_tail_lsn(mp: *mut xfs_mount) -> xfs_lsn_t;
}
extern "C" {
    pub fn xlog_assign_tail_lsn_locked(mp: *mut xfs_mount) -> xfs_lsn_t;
}
extern "C" {
    pub fn xfs_log_space_wake(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_log_regrant(mp: *mut xfs_mount, tic: *mut xlog_ticket) -> c_int;
}
extern "C" {
    pub fn xfs_log_unmount(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_log_writable(mp: *mut xfs_mount) -> bool;
}
extern "C" {
    pub fn xfs_log_ticket_put(ticket: *mut xlog_ticket);
}
extern "C" {
    pub fn xlog_cil_process_committed(list: *mut list_head);
}
extern "C" {
    pub fn xfs_log_item_in_current_chkpt(lip: *mut xfs_log_item) -> bool;
}
extern "C" {
    pub fn xfs_log_work_queue(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_log_quiesce(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_log_clean(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_log_check_lsn(: *mut xfs_mount, _arg: xfs_lsn_t) -> bool;
}
extern "C" {
    pub fn xlog_force_shutdown(log: *mut xlog, shutdown_flags: u32) -> bool;
}
