//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_dir2.h
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
// Copyright (c) 2000-2001,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfs_dir2_fmt {
    XFS_DIR2_FMT_SF,
    XFS_DIR2_FMT_BLOCK,
    XFS_DIR2_FMT_LEAF,
    XFS_DIR2_FMT_NODE,
    XFS_DIR2_FMT_ERROR,
}

extern "C" {
    pub fn xfs_dir2_format(args: *mut xfs_da_args, error: *mut c_int) -> xfs_dir2_fmt;
}
//
// Convert inode mode to directory entry filetype
//
extern "C" {
    pub fn xfs_mode_to_ftype(mode: c_int) -> c_uchar;
}
//
// Generic directory interface routines
//
extern "C" {
    pub fn xfs_dir_startup();
}
extern "C" {
    pub fn xfs_da_mount(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_da_unmount(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_dir_lookup_args(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir_createname_args(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir_removename_args(args: *mut xfs_da_args) -> c_int;
}
extern "C" {
    pub fn xfs_dir_replace_args(args: *mut xfs_da_args) -> c_int;
}
//
// Direct call from the bmap code, bypassing the generic directory layer.
//
extern "C" {
    pub fn xfs_dir2_sf_to_block(args: *mut xfs_da_args) -> c_int;
}
//
// Interface routines used by userspace utilities
//
extern "C" {
    pub fn xfs_dir_ino_validate(mp: *mut xfs_mount, ino: xfs_ino_t) -> c_int;
}
extern "C" {
    pub fn xfs_dir3_leaf_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
}
extern "C" {
    pub fn xfs_dir3_data_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
}
extern "C" {
    pub fn xfs_dir3_block_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
}
//
// Directory offset/block conversion functions.
//
// DB blocks here are logical directory block numbers, not filesystem blocks.
//
// Convert dataptr to byte in file space
//
// Convert byte in file space to dataptr.  It had better be aligned.
//
// Convert byte in space to (DB) block
//
// Convert dataptr to a block number
//
extern "C" {
    pub fn xfs_dir2_byte_to_db(_arg: geo, _arg: xfs_dir2_dataptr_to_byte(dp)) -> return;
}
//
// Convert byte in space to offset in a block
//
// Convert dataptr to a byte offset in a block
//
extern "C" {
    pub fn xfs_dir2_byte_to_off(_arg: geo, _arg: xfs_dir2_dataptr_to_byte(dp)) -> return;
}
//
// Convert block and offset to byte in space
//
// Convert block (DB) to block (dablk)
//
// Convert byte in space to (DA) block
//
extern "C" {
    pub fn xfs_dir2_db_to_da(_arg: geo, _arg: xfs_dir2_byte_to_db(geo, _arg: by)) -> return;
}
//
// Convert block and offset to dataptr
//
extern "C" {
    pub fn xfs_dir2_byte_to_dataptr(_arg: xfs_dir2_db_off_to_byte(geo, _arg: db, _arg: o)) -> return;
}
//
// Convert block (dablk) to block (DB)
//
// Convert block (dablk) to byte offset in space
//
extern "C" {
    pub fn xfs_dir2_db_off_to_byte(_arg: geo, _arg: xfs_dir2_da_to_db(geo, _arg: da), _arg: 0) -> return;
}
//
// Directory tail pointer accessor functions. Based on block geometry.
//
// The Linux API doesn't pass down the total size of the buffer
// we read into down to the filesystem.  With the filldir concept
// it's not needed for correct information, but the XFS dir2 leaf
// code wants an estimate of the buffer size to calculate it's
// readahead window and size the buffers used for mapping to
// physical blocks.
//
// Try to give it an estimate that's good enough, maybe at some
// point we can change the ->readdir prototype to include the
// buffer size.  For now we use the current glibc buffer size.
// musl libc hardcodes 2k and dietlibc uses PAGE_SIZE.
//

extern "C" {
    pub fn xfs_dir3_get_dtype(mp: *mut xfs_mount, filetype: u8) -> c_uchar;
}
extern "C" {
    pub fn xfs_dir2_namecheck(name: *const c_void, length: usize) -> bool;
}
//
// The "ascii-ci" feature was created to speed up case-insensitive lookups for
// a Samba product.  Because of the inherent problems with CI and UTF-8
// encoding, etc, it was decided that Samba would be configured to export
// latin1/iso 8859-1 encodings as that covered >90% of the target markets for
// the product.  Hence the "ascii-ci" casefolding code could be encoded into
// the XFS directory operations and remove all the overhead of casefolding from
// Samba.
//
// To provide consistent hashing behavior between the userspace and kernel,
// these functions prepare names for hashing by transforming specific bytes
// to other bytes.  Robustness with other encodings is not guaranteed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir_update_params {
    pub dp: *const xfs_inode,
    pub ip: *const xfs_inode,
    pub name: *const xfs_name,
    pub delta: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir_hook {
    pub dirent_hook: xfs_hook,
}

extern "C" {
    pub fn xfs_dir_hook_disable();
}
extern "C" {
    pub fn xfs_dir_hook_enable();
}
extern "C" {
    pub fn xfs_dir_hook_add(mp: *mut xfs_mount, hook: *mut xfs_dir_hook) -> c_int;
}
extern "C" {
    pub fn xfs_dir_hook_del(mp: *mut xfs_mount, hook: *mut xfs_dir_hook);
}
extern "C" {
    pub fn xfs_dir_hook_setup(hook: *mut xfs_dir_hook, mod_fn: notifier_fn_t);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_dir_update {
    pub dp: *mut xfs_inode,
    pub name: *const xfs_name,
    pub ip: *mut xfs_inode,
    pub ppargs: *mut xfs_parent_args,
}
