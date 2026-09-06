//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/sufile.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// NILFS segment usage file.
//
// Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Koji Sato.
//

extern "C" {
    pub fn nilfs_sufile_get_ncleansegs(sufile: *mut inode) -> c_ulong;
}
extern "C" {
    pub fn nilfs_sufile_set_alloc_range(sufile: *mut inode, start: __u64, end: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_sufile_alloc(: *mut inode, : *mut __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_sufile_mark_dirty(sufile: *mut inode, segnum: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_sufile_get_stat(: *mut inode, : *mut nilfs_sustat) -> c_int;
}
extern "C" {
    pub fn nilfs_sufile_set_suinfo(: *mut inode, : *mut c_void, int: unsigned, _arg: usize) -> isize;
}
extern "C" {
    pub fn nilfs_sufile_resize(sufile: *mut inode, newnsegs: __u64) -> c_int;
}
extern "C" {
    pub fn nilfs_sufile_trim_fs(sufile: *mut inode, range: *mut fstrim_range) -> c_int;
}
//
// nilfs_sufile_warn_on_error - warn on unexpected sufile error
// @sufile: inode of segment usage file
// @err: status code returned by a sufile function
//
// Even if buffer heads of blocks containing segment usage entries have
// been dirtied in advance by calling functions such as
// nilfs_sufile_mark_dirty() or nilfs_sufile_{alloc,free}(), those buffers
// can be discarded from memory after the file system detects corruption and
// degrades to read-only mode, which may cause sufile operations, including
// cancel operations, to return errors.  nilfs_sufile_warn_on_error() is used
// to detect unexpected errors other than during read-only degradation.
//
// Return: 0 if @err is 0, %-EROFS if in read-only degraded mode, and %-EIO
// otherwise.
//

//
// nilfs_sufile_scrap - make a segment garbage
// @sufile: inode of segment usage file
// @segnum: segment number to be freed
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn nilfs_sufile_update(_arg: sufile, _arg: segnum, _arg: 1, _arg: nilfs_sufile_do_scrap) -> return;
}
//
// nilfs_sufile_free - free segment
// @sufile: inode of segment usage file
// @segnum: segment number to be freed
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn nilfs_sufile_update(_arg: sufile, _arg: segnum, _arg: 0, _arg: nilfs_sufile_do_free) -> return;
}
//
// nilfs_sufile_freev - free segments
// @sufile: inode of segment usage file
// @segnumv: array of segment numbers
// @nsegs: size of @segnumv array
// @ndone: place to store the number of freed segments
//
// Return: 0 on success, or a negative error code on failure.
//
// nilfs_sufile_cancel_freev - reallocate freeing segments
// @sufile: inode of segment usage file
// @segnumv: array of segment numbers
// @nsegs: size of @segnumv array
// @ndone: place to store the number of cancelled segments
//
// Return: 0 on success, or a negative error code on failure.
//
// nilfs_sufile_set_error - mark a segment as erroneous
// @sufile: inode of segment usage file
// @segnum: segment number
//
// Description: nilfs_sufile_set_error() marks the segment specified by
// @segnum as erroneous. The error segment will never be used again.
//
// Return: 0 on success, or one of the following negative error codes on
// failure:
// * %-EINVAL	- Invalid segment usage number.
// * %-EIO	- I/O error (including metadata corruption).
// * %-ENOMEM	- Insufficient memory available.
//
