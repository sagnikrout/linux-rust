//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_dtree.h
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
// jfs_dtree.h: directory B+-tree manager
//

//
// entry segment/slot
//
// an entry consists of type dependent head/only segment/slot and
// additional segments/slots linked vi next field;
// N.B. last/only segment of entry is terminated by next = -1;
//
// directory page slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtslot {
    pub /: *mut *mut s8 next; / 1:,
    pub /: *mut *mut s8 cnt; / 1:,
    pub /: *mut *mut __le16 name[15]; / 30:,
}

pub const DATASLOTSIZE: c_int = 16;
pub const L2DATASLOTSIZE: c_int = 4;
pub const DTSLOTSIZE: c_int = 32;
pub const L2DTSLOTSIZE: c_int = 5;
pub const DTSLOTHDRSIZE: c_int = 2;
pub const DTSLOTDATASIZE: c_int = 30;
pub const DTSLOTDATALEN: c_int = 15;
//
// internal node entry head/only segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtentry {
    pub /: *mut *mut pxd_t xd; / 8: child extent descriptor,
    pub /: *mut *mut s8 next; / 1:,
    pub /: *mut *mut u8 namlen; / 1:,
    pub /: *mut *mut __le16 name[11]; / 22: 2-byte aligned,
}

pub const DTIHDRSIZE: c_int = 10;
pub const DTIHDRDATALEN: c_int = 11;
// compute number of slots for entry

//
// leaf node entry head/only segment
//
// For legacy filesystems, name contains 13 wchars -- no index field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldtentry {
    pub /: *mut *mut __le32 inumber; / 4: 4-byte aligned,
    pub /: *mut *mut s8 next; / 1:,
    pub /: *mut *mut u8 namlen; / 1:,
    pub /: *mut *mut __le16 name[11]; / 22: 2-byte aligned,
    pub /: *mut *mut __le32 index; / 4: index into dir_table,
}

pub const DTLHDRSIZE: c_int = 6;

pub const DTLHDRDATALEN: c_int = 11;
//
// dir_table used for directory traversal during readdir
//
// Keep persistent index for directory entries
//

//
// Maximum entry in inline directory table
//
pub const MAX_INLINE_DIRTABLE_ENTRY: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir_table_slot {
    pub /: *mut *mut u8 rsrvd; / 1:,
    pub /: *mut *mut u8 flag; / 1: 0 if free,
    pub /: *mut *mut u8 slot; / 1: slot within leaf page of entry,
    pub /: *mut *mut u8 addr1; / 1: upper 8 bits of leaf page address,
    pub -OR-: *mut *mut __le32 addr2; / 4: lower 32 bits of leaf page address,
}

//
// flag values
//
pub const DIR_INDEX_VALID: c_int = 1;
pub const DIR_INDEX_FREE: c_int = 0;

// Macro flag: #define addressDTS(dts)\
// compute number of slots for entry

//
// directory root page (in-line in on-disk inode):
//
// cf. dtpage_t below.
//

pub const DTROOTMAXSLOT: c_int = 9;

//
// directory regular page:
//
// entry slot array of 32 byte slot
//
// sorted entry slot index table (stbl):
// contiguous slots at slot specified by stblindex,
// 1-byte per entry
// 512 byte block:  16 entry tbl (1 slot)
// 1024 byte block:  32 entry tbl (1 slot)
// 2048 byte block:  64 entry tbl (2 slot)
// 4096 byte block: 128 entry tbl (4 slot)
//
// data area:
// 512 byte block:  16 - 2 =  14 slot
// 1024 byte block:  32 - 2 =  30 slot
// 2048 byte block:  64 - 3 =  61 slot
// 4096 byte block: 128 - 5 = 123 slot
//
// N.B. index is 0-based; index fields refer to slot index
// except nextindex which refers to entry index in stbl;
// end of entry stot list or freelist is marked with -1.
//
pub const DTPAGEMAXSLOT: c_int = 128;
pub const DT8THPGNODEBYTES: c_int = 512;
pub const DT8THPGNODETSLOTS: c_int = 1;
pub const DT8THPGNODESLOTS: c_int = 16;
pub const DTQTRPGNODEBYTES: c_int = 1024;
pub const DTQTRPGNODETSLOTS: c_int = 1;
pub const DTQTRPGNODESLOTS: c_int = 32;
pub const DTHALFPGNODEBYTES: c_int = 2048;
pub const DTHALFPGNODETSLOTS: c_int = 2;
pub const DTHALFPGNODESLOTS: c_int = 64;
pub const DTFULLPGNODEBYTES: c_int = 4096;
pub const DTFULLPGNODETSLOTS: c_int = 4;
pub const DTFULLPGNODESLOTS: c_int = 128;
pub const DTENTRYSTART: c_int = 1;
// get sorted entry table of the page

//
// Flags for dtSearch
//
pub const JFS_CREATE: c_int = 1;
pub const JFS_LOOKUP: c_int = 2;
pub const JFS_REMOVE: c_int = 3;
pub const JFS_RENAME: c_int = 4;
//
// Maximum file offset for directories.
//

//
// external declarations
//
extern "C" {
    pub fn dtInitRoot(tid: tid_t, ip: *mut inode, idotdot: u32);
}
extern "C" {
    pub fn jfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
}
extern "C" {
    pub fn check_dtroot(p: *mut dtroot_t) -> bool;
}
extern "C" {
    pub fn check_dtpage(p: *mut dtpage_t) -> bool;
}
