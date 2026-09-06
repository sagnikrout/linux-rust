//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_iunlink_item.h
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
// Copyright (c) 2020-2022, Red Hat, Inc.
// All Rights Reserved.
//
pub const XFS_IUNLINK_ITEM_H: c_int = 1;
// in memory log item structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_iunlink_item {
    pub item: xfs_log_item,
    pub ip: *mut xfs_inode,
    pub pag: *mut xfs_perag,
    pub next_agino: xfs_agino_t,
    pub old_agino: xfs_agino_t,
}
