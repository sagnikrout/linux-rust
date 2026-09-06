//! Automatically rewritten from C Header to Rust Module
//! Source: fs/gfs2/trans.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
// Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
//

pub const RES_DINODE: c_int = 1;
pub const RES_INDIRECT: c_int = 1;
pub const RES_JDATA: c_int = 1;
pub const RES_DATA: c_int = 1;
pub const RES_LEAF: c_int = 1;
pub const RES_RG_HDR: c_int = 1;
pub const RES_RG_BIT: c_int = 2;
pub const RES_EATTR: c_int = 1;
pub const RES_STATFS: c_int = 1;
pub const RES_QUOTA: c_int = 2;
// reserve either the number of blocks to be allocated plus the rg header
// block, or all of the blocks in the rg, whichever is smaller
extern "C" {
    pub fn gfs2_trans_end(sdp: *mut gfs2_sbd);
}
extern "C" {
    pub fn gfs2_trans_add_data(gl: *mut gfs2_glock, bh: *mut buffer_head);
}
extern "C" {
    pub fn gfs2_trans_add_meta(gl: *mut gfs2_glock, bh: *mut buffer_head);
}
extern "C" {
    pub fn gfs2_trans_add_revoke(sdp: *mut gfs2_sbd, bd: *mut gfs2_bufdata);
}
extern "C" {
    pub fn gfs2_trans_remove_revoke(sdp: *mut gfs2_sbd, blkno: u64, len: c_uint);
}
extern "C" {
    pub fn gfs2_trans_free(sdp: *mut gfs2_sbd, tr: *mut gfs2_trans);
}
