//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_btree.h
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
// Copyright (C) International Business Machines Corp., 2000-2004
//
// jfs_btree.h: B+-tree
//
// JFS B+-tree (dtree and xtree) common definitions
//
// basic btree page - btpage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btpage {
    pub bn: s64 next; right sibling,
    pub bn: s64 prev; left sibling,
    pub flag: u8,
    pub specific: u8 rsrvd[7]; type,
    pub address: s64 self; self,
    pub entry: [u8; 4064],
}

// btpaget_t flag
pub const BT_TYPE: c_uint = 0x07	/* B+-tree index */;
pub const BT_ROOT: c_uint = 0x01	/* root page */;
pub const BT_LEAF: c_uint = 0x02	/* leaf page */;
pub const BT_INTERNAL: c_uint = 0x04	/* internal page */;
pub const BT_RIGHTMOST: c_uint = 0x10	/* rightmost page */;
pub const BT_LEFTMOST: c_uint = 0x20	/* leftmost page */;
pub const BT_SWAPPED: c_uint = 0x80	/* used by fsck for endian swapping */;
// btorder (in inode)
pub const BT_RANDOM: c_uint = 0x0000;
pub const BT_SEQUENTIAL: c_uint = 0x0001;
pub const BT_LOOKUP: c_uint = 0x0010;
pub const BT_INSERT: c_uint = 0x0020;
pub const BT_DELETE: c_uint = 0x0040;
//
// btree page buffer cache access
//

// get page from buffer page

// get the page buffer and the page for specified block address

// put the page buffer
// Macro flag: #define BT_PUTPAGE(MP)\
//
// btree traversal stack
//
// record the path traversed during the search;
// top frame record the leaf page/entry selected.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btframe {
    pub /: *mut *mut s64 bn; / 8:,
    pub /: *mut *mut s16 index; / 2:,
    pub /: *mut *mut s16 lastindex; / 2: unused,
    pub /: *mut *mut *mut metapage mp; / 4/8:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btstack {
    pub top: *mut btframe,
    pub nsplit: c_int,
    pub stack: [btframe; MAXTREEHEIGHT],
}

// Macro flag: #define BT_CLR(btstack)\
// Macro flag: #define BT_STACK_FULL(btstack)\

// Macro flag: #define BT_POP(btstack)\
// Macro flag: #define BT_STACK(btstack)\
// retrieve search results

// put the page buffer of search
// Macro flag: #define BT_PUTSEARCH(BTSTACK)\
