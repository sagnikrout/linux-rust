//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ubifs/misc.h
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
// This file is part of UBIFS.
//
// Copyright (C) 2006-2008 Nokia Corporation
//
// Authors: Artem Bityutskiy (Битюцкий Артём)
// Adrian Hunter
//
// This file contains miscellaneous helper functions.
//
// ubifs_zn_dirty - check if znode is dirty.
// @znode: znode to check
//
// This helper function returns %1 if @znode is dirty and %0 otherwise.
//
// ubifs_zn_obsolete - check if znode is obsolete.
// @znode: znode to check
//
// This helper function returns %1 if @znode is obsolete and %0 otherwise.
//
// ubifs_zn_cow - check if znode has to be copied on write.
// @znode: znode to check
//
// This helper function returns %1 if @znode is has COW flag set and %0
// otherwise.
//
// ubifs_wake_up_bgt - wake up background thread.
// @c: UBIFS file-system description object
//
// ubifs_tnc_find_child - find next child in znode.
// @znode: znode to search at
// @start: the zbranch index to start at
//
// This helper function looks for znode child starting at index @start. Returns
// the child or %NULL if no children were found.
//
// ubifs_inode - get UBIFS inode information by VFS 'struct inode' object.
// @inode: the VFS 'struct inode' pointer
//
extern "C" {
    pub fn container_of(_arg: inode, ubifs_inode: struct, _arg: vfs_inode) -> return;
}
//
// ubifs_compr_present - check if compressor was compiled in.
// @compr_type: compressor type to check
// @c: the UBIFS file-system description object
//
// This function returns %1 of compressor of type @compr_type is present, and
// %0 if not.
//
// ubifs_compr_name - get compressor name string by its type.
// @compr_type: compressor type
// @c: the UBIFS file-system description object
//
// This function returns compressor type string.
//
// ubifs_wbuf_sync - synchronize write-buffer.
// @wbuf: write-buffer to synchronize
//
// This is the same as 'ubifs_wbuf_sync_nolock()' but it does not assume
// that the write-buffer is already locked.
//
// ubifs_encode_dev - encode device node IDs.
// @dev: UBIFS device node information
// @rdev: device IDs to encode
//
// This is a helper function which encodes major/minor numbers of a device node
// into UBIFS device node description. We use standard Linux "new" and "huge"
// encodings.
//
extern "C" {
    pub fn sizeof(_arg: dev->new) -> return;
}
//
// ubifs_add_dirt - add dirty space to LEB properties.
// @c: the UBIFS file-system description object
// @lnum: LEB to add dirty space for
// @dirty: dirty space to add
//
// This is a helper function which increased amount of dirty LEB space. Returns
// zero in case of success and a negative error code in case of failure.
//
extern "C" {
    pub fn ubifs_update_one_lp(_arg: c, _arg: lnum, _arg: LPROPS_NC, _arg: dirty, _arg: 0, _arg: 0) -> return;
}
//
// ubifs_return_leb - return LEB to lprops.
// @c: the UBIFS file-system description object
// @lnum: LEB to return
//
// This helper function cleans the "taken" flag of a logical eraseblock in the
// lprops. Returns zero in case of success and a negative error code in case of
// failure.
//
// ubifs_idx_node_sz - return index node size.
// @c: the UBIFS file-system description object
// @child_cnt: number of children of this index node
//
// child_cnt;
//
// ubifs_idx_branch - return pointer to an index branch.
// @c: the UBIFS file-system description object
// @idx: index node
// @bnum: branch number
//
// ubifs_idx_key - return pointer to an index key.
// @c: the UBIFS file-system description object
// @idx: index node
//
// ubifs_tnc_lookup - look up a file-system node.
// @c: UBIFS file-system description object
// @key: node key to lookup
// @node: the node is returned here
//
// This function look up and reads node with key @key. The caller has to make
// sure the @node buffer is large enough to fit the node. Returns zero in case
// of success, %-ENOENT if the node was not found, and a negative error code in
// case of failure.
//
extern "C" {
    pub fn ubifs_tnc_locate(_arg: c, _arg: key, _arg: node, _arg: NULL, _arg: NULL) -> return;
}
//
// ubifs_get_lprops - get reference to LEB properties.
// @c: the UBIFS file-system description object
//
// This function locks lprops. Lprops have to be unlocked by
// 'ubifs_release_lprops()'.
//
// ubifs_release_lprops - release lprops lock.
// @c: the UBIFS file-system description object
//
// This function has to be called after each 'ubifs_get_lprops()' call to
// unlock lprops.
//
// ubifs_next_log_lnum - switch to the next log LEB.
// @c: UBIFS file-system description object
// @lnum: current log LEB
//
// This helper function returns the log LEB number which goes next after LEB
// 'lnum'.
//
