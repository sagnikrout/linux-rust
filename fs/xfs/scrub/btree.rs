//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/btree.h
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
// Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// btree scrub
// Check for btree operation errors.
// Check for btree xref operation errors.
// Check for btree corruption.
// Check for btree xref discrepancies.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_btree_key {
    pub key: xfs_btree_key,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_btree {
// caller-provided scrub state
    pub sc: *mut xfs_scrub,
    pub cur: *mut xfs_btree_cur,
    pub scrub_rec: xchk_btree_rec_fn,
    pub oinfo: *const xfs_owner_info,
    pub private: *mut c_void,
// internal scrub state
    pub lastrec_valid: bool,
    pub lastrec: xfs_btree_rec,
    pub to_check: list_head,
// this element must come last!
    pub lastkey: [xchk_btree_key; ],
}

//
// Calculate the size of a xchk_btree structure.  There are nlevels-1 slots for
// keys because we track leaf records separately in lastrec.
//
extern "C" {
    pub fn struct_size_t(xchk_btree: struct, _arg: lastkey, 1: nlevels -) -> return;
}
