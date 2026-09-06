//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_xtree.h
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
// Copyright (C) International Business Machines Corp., 2000-2002
//
// jfs_xtree.h: extent allocation descriptor B+-tree manager
//

//
// extent allocation descriptor (xad)
//

pub const XTSLOTSIZE: c_int = 16;
pub const L2XTSLOTSIZE: c_int = 4;
// xad_t field construction

// xad_t field extraction
// Macro flag: #define offsetXAD(xad)\

// xad list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xadlist {
    pub maxnxad: i16,
    pub nxad: i16,
    pub xad: *mut xad_t,
}

// xad_t flags
pub const XAD_NEW: c_uint = 0x01	/* new */;
pub const XAD_EXTENDED: c_uint = 0x02	/* extended */;
pub const XAD_COMPRESSED: c_uint = 0x04	/* compressed with recorded length */;
pub const XAD_NOTRECORDED: c_uint = 0x08	/* allocated but not recorded */;
pub const XAD_COW: c_uint = 0x10	/* copy-on-write */;
// possible values for maxentry
pub const XTROOTINITSLOT_DIR: c_int = 6;
pub const XTROOTINITSLOT: c_int = 10;
pub const XTROOTMAXSLOT: c_int = 18;
pub const XTPAGEMAXSLOT: c_int = 256;
pub const XTENTRYSTART: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtheader {
    pub /: *mut *mut __le64 next; / 8:,
    pub /: *mut *mut __le64 prev; / 8:,
    pub /: *mut *mut u8 flag; / 1:,
    pub /: *mut *mut u8 rsrvd1; / 1:,
    pub /: *mut *mut __le16 nextindex; / 2: next index = number of entries,
    pub /: *mut *mut __le16 maxentry; / 2: max number of entries,
    pub /: *mut *mut __le16 rsrvd2; / 2:,
    pub /: *mut *mut pxd_t self; / 8: self,
}

//
// xtree root (in inode):
//
// xtree page:
//
// external declaration
//
extern "C" {
    pub fn xtInitRoot(tid: tid_t, ip: *mut inode);
}
extern "C" {
    pub fn xtUpdate(tid: tid_t, ip: *mut inode, nxad: *mut xad) -> c_int;
}
extern "C" {
    pub fn xtTruncate(tid: tid_t, ip: *mut inode, newsize: i64, type: c_int) -> i64;
}
extern "C" {
    pub fn xtTruncate_pmap(tid: tid_t, ip: *mut inode, committed_size: i64) -> i64;
}
