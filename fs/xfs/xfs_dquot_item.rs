//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_dquot_item.h
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
// Copyright (c) 2000-2003 Silicon Graphics, Inc.
// All Rights Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dq_logitem {
    pub /: *mut *mut xfs_log_item qli_item; / common portion,
    pub /: *mut *mut *mut xfs_dquot qli_dquot; / dquot ptr,
    pub /: *mut *mut xfs_lsn_t qli_flush_lsn; / lsn at last flush,
//
// We use this spinlock to coordinate access to the li_buf pointer in
// the log item and the qli_dirty flag.
//
    pub qli_lock: spinlock_t,
    pub /: *mut *mut bool qli_dirty; / dirtied since last flush?,
}

extern "C" {
    pub fn xfs_qm_dquot_logitem_init(dqp: *mut xfs_dquot);
}
