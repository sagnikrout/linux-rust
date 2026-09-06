//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/mft.h
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
// Defines for mft record handling in NTFS Linux kernel driver.
//
// Copyright (c) 2001-2004 Anton Altaparmakov
//

extern "C" {
    pub fn unmap_mft_record(ni: *mut ntfs_inode);
}
extern "C" {
    pub fn __mark_mft_record_dirty(ni: *mut ntfs_inode);
}
//
// mark_mft_record_dirty - set the mft record and the page containing it dirty
// @ni:		ntfs inode describing the mapped mft record
//
// Set the mapped (extent) mft record of the (base or extent) ntfs inode @ni,
// as well as the page containing the mft record, dirty.  Also, mark the base
// vfs inode dirty.  This ensures that any changes to the mft record are
// written out to disk.
//
// NOTE:  Do not do anything if the mft record is already marked dirty.
//
extern "C" {
    pub fn write_mft_record_nolock(ni: *mut ntfs_inode, m: *mut mft_record, sync: c_int) -> c_int;
}
//
// write_mft_record - write out a mapped (extent) mft record
// @ni:		ntfs inode describing the mapped (extent) mft record
// @m:		mapped (extent) mft record to write
// @sync:	if true, wait for i/o completion
//
// This is just a wrapper for write_mft_record_nolock() (see mft.c), which
// locks the page for the duration of the write.  This ensures that there are
// no race conditions between writing the mft record via the dirty inode code
// paths and via the page cache write back code paths or between writing
// neighbouring mft records residing in the same page.
//
// Locking the page also serializes us against ->read_folio() if the page is not
// uptodate.
//
// On success, clean the mft record and return 0.  On error, leave the mft
// record dirty and return -errno.
//
extern "C" {
    pub fn ntfs_mft_record_free(vol: *mut ntfs_volume, ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ntfs_mft_mark_dirty(folio: *mut folio);
}
