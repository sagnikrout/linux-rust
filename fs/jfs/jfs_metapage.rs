//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_metapage.h
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
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct metapage {
// Common logsyncblk prefix (see jfs_logmgr.h)
    pub xflag: u16,
    pub unused: u16,
    pub lid: lid_t,
    pub lsn: c_int,
    pub synclist: list_head,
// End of logsyncblk prefix
    pub /: *mut *mut unsigned long flag; / See Below,
    pub /: *mut *mut unsigned long count; / Reference count,
    pub /: *mut *mut *mut void data; / Data pointer,
    pub /: *mut *mut sector_t index; / block address of page,
    pub wait: wait_queue_head_t,
// implementation
    pub folio: *mut folio,
    pub sb: *mut super_block,
    pub logical_size: c_uint,
// Journal management
    pub clsn: c_int,
    pub nohomeok: c_int,
    pub log: *mut jfs_log,
}

// metapage flag
pub const META_locked: c_int = 0;
pub const META_dirty: c_int = 2;
pub const META_sync: c_int = 3;
pub const META_discard: c_int = 4;
pub const META_forcewrite: c_int = 5;
pub const META_io: c_int = 6;

// function prototypes
extern "C" {
    pub fn metapage_init() -> c_int;
}
extern "C" {
    pub fn metapage_exit();
}

extern "C" {
    pub fn release_metapage(: *mut metapage);
}
extern "C" {
    pub fn grab_metapage(: *mut metapage);
}
extern "C" {
    pub fn force_metapage(: *mut metapage);
}
//
// hold_metapage and put_metapage are used in conjunction.  The page lock
// is not dropped between the two, so no other threads can get or release
// the metapage
//
extern "C" {
    pub fn hold_metapage(: *mut metapage);
}
extern "C" {
    pub fn put_metapage(: *mut metapage);
}
//
// This serializes access to mp->lsn when metapages are added to logsynclist
// without setting nohomeok.  i.e. updating imap & dmap
//
// This is called when already holding the metapage
//
// This routines invalidate all pages for an extent.
//
extern "C" {
    pub fn __invalidate_metapages(: *mut inode, _arg: i64, _arg: c_int);
}

