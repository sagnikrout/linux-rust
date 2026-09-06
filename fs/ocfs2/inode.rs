//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/inode.h
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
// inode.h
//
// Function prototypes
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//

// OCFS2 Inode Private Data
// protects allocation changes on this inode.
// protects extended attribute changes on this inode
// These fields are protected by ip_lock
// Record unwritten extents during direct io.
// protected by recovery_lock.
// Only valid if the inode is the dir.
//
// Transactions that contain inode's metadata needed to complete
// fsync and fdatasync, respectively.
//
// Flags for the ip_flags field
//
// System file inodes
pub const OCFS2_INODE_SYSTEM_FILE: c_uint = 0x00000001;
pub const OCFS2_INODE_JOURNAL: c_uint = 0x00000002;
pub const OCFS2_INODE_BITMAP: c_uint = 0x00000004;
// This inode has been wiped from disk
pub const OCFS2_INODE_DELETED: c_uint = 0x00000008;
// Has the inode been orphaned on another node?
//
// This hints to ocfs2_drop_inode that it should clear i_nlink before
// continuing.
//
// We *only* set this on unlink vote from another node. If the inode
// was locally orphaned, then we're sure of the state and don't need
// to twiddle i_nlink later - it's either zero or not depending on
// whether our unlink succeeded. Otherwise we got this from a node
// whose intention was to orphan the inode, however he may have
// crashed, failed etc, so we let ocfs2_drop_inode zero the value and
// rely on ocfs2_delete_inode to sort things out under the proper
// cluster locks.
//
pub const OCFS2_INODE_MAYBE_ORPHANED: c_uint = 0x00000010;
// Does someone have the file open O_DIRECT
pub const OCFS2_INODE_OPEN_DIRECT: c_uint = 0x00000020;
// Tell the inode wipe code it's not in orphan dir
pub const OCFS2_INODE_SKIP_ORPHAN_DIR: c_uint = 0x00000040;
// Entry in orphan dir with 'dio-' prefix
pub const OCFS2_INODE_DIO_ORPHAN_ENTRY: c_uint = 0x00000080;
extern "C" {
    pub fn container_of(_arg: inode, ocfs2_inode_info: struct, _arg: vfs_inode) -> return;
}

extern "C" {
    pub fn ocfs2_evict_inode(inode: *mut inode);
}
// Flags for ocfs2_iget()
pub const OCFS2_FI_FLAG_SYSFILE: c_uint = 0x1;
pub const OCFS2_FI_FLAG_ORPHAN_RECOVERY: c_uint = 0x2;
pub const OCFS2_FI_FLAG_FILECHECK_CHK: c_uint = 0x4;
pub const OCFS2_FI_FLAG_FILECHECK_FIX: c_uint = 0x8;
extern "C" {
    pub fn ocfs2_inode_revalidate(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ocfs2_sync_blockdev(sb: *mut super_block);
}
extern "C" {
    pub fn ocfs2_set_inode_flags(inode: *mut inode);
}
extern "C" {
    pub fn ocfs2_get_inode_flags(oi: *mut ocfs2_inode_info);
}
// Validate that a bh contains a valid inode
//
// Read an inode block into *bh.  If *bh is NULL, a bh will be allocated.
// This is a cached read.  The inode will be validated with
// ocfs2_validate_inode_block().
//
extern "C" {
    pub fn ocfs2_read_inode_block(inode: *mut inode, bh: *mut buffer_head) -> c_int;
}
// The same, but can be passed OCFS2_BH_* flags
extern "C" {
    pub fn container_of(_arg: ci, ocfs2_inode_info: struct, _arg: ip_metadata_cache) -> return;
}
// Does this inode have the reflink flag set?
