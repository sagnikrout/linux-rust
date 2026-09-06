//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_exchmaps_item.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// The file mapping exchange intent item helps us exchange multiple file
// mappings between two inode forks.  It does this by tracking the range of
// file block offsets that still need to be exchanged, and relogs as progress
// happens.
//
// *I items should be recorded in the *first* of a series of rolled
// transactions, and the *D items should be recorded in the same transaction
// that records the associated bmbt updates.
//
// Should the system crash after the commit of the first transaction but
// before the commit of the final transaction in a series, log recovery will
// use the redo information recorded by the intent items to replay the
// rest of the mapping exchanges.
//
// kernel only XMI/XMD definitions
//
// This is the incore file mapping exchange intent log item.  It is used to log
// the fact that we are exchanging mappings between two files.  It is used in
// conjunction with the incore file mapping exchange done log item described
// below.
//
// These log items follow the same rules as struct xfs_efi_log_item; see the
// comments about that structure (in xfs_extfree_item.h) for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_xmi_log_item {
    pub xmi_item: xfs_log_item,
    pub xmi_refcount: core::sync::atomic::AtomicI32,
    pub xmi_format: xfs_xmi_log_format,
}

//
// This is the incore file mapping exchange done log item.  It is used to log
// the fact that an exchange mentioned in an earlier xmi item have been
// performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_xmd_log_item {
    pub xmd_item: xfs_log_item,
    pub xmd_intent_log_item: *mut xfs_xmi_log_item,
    pub xmd_format: xfs_xmd_log_format,
}
