//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/dlmglue.h
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
// dlmglue.h
//
// description here
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//

pub const OCFS2_LVB_VERSION: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_meta_lvb {
    pub lvb_version: __u8,
    pub lvb_reserved0: __u8,
    pub lvb_idynfeatures: __be16,
    pub lvb_iclusters: __be32,
    pub lvb_iuid: __be32,
    pub lvb_igid: __be32,
    pub lvb_iatime_packed: __be64,
    pub lvb_ictime_packed: __be64,
    pub lvb_imtime_packed: __be64,
    pub lvb_isize: __be64,
    pub lvb_imode: __be16,
    pub lvb_inlink: __be16,
    pub lvb_iattr: __be32,
    pub lvb_igeneration: __be32,
    pub lvb_reserved2: __be32,
}

pub const OCFS2_QINFO_LVB_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_qinfo_lvb {
    pub lvb_version: __u8,
    pub lvb_reserved: [__u8; 3],
    pub lvb_bgrace: __be32,
    pub lvb_igrace: __be32,
    pub lvb_syncms: __be32,
    pub lvb_blocks: __be32,
    pub lvb_free_blk: __be32,
    pub lvb_free_entry: __be32,
}

pub const OCFS2_ORPHAN_LVB_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_orphan_scan_lvb {
    pub lvb_version: __u8,
    pub lvb_reserved: [__u8; 3],
    pub lvb_os_seqno: __be32,
}

pub const OCFS2_TRIMFS_LVB_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_trim_fs_lvb {
    pub lvb_version: __u8,
    pub lvb_success: __u8,
    pub lvb_reserved: [__u8; 2],
    pub lvb_nodenum: __be32,
    pub lvb_start: __be64,
    pub lvb_len: __be64,
    pub lvb_minlen: __be64,
    pub lvb_trimlen: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_trim_fs_info {
    pub /: *mut *mut u8 tf_valid; / lvb is valid, or not,
    pub /: *mut *mut u8 tf_success; / trim is successful, or not,
    pub /: *mut *mut u32 tf_nodenum; / osb node number,
    pub /: *mut *mut u64 tf_start; / trim start offset in clusters,
    pub /: *mut *mut u64 tf_len; / trim end offset in clusters,
    pub /: *mut *mut u64 tf_minlen; / trim minimum contiguous free clusters,
    pub /: *mut *mut u64 tf_trimlen; / trimmed length in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_lock_holder {
    pub oh_list: list_head,
    pub oh_owner_pid: *mut pid,
    pub oh_ex: c_int,
}

// ocfs2_inode_lock_full() 'arg_flags' flags
// don't wait on recovery.

// Instruct the dlm not to queue ourselves on the other node.

// don't block waiting for the downconvert thread, instead return -EAGAIN

// just get back disk inode bh if we've got cluster lock.

// Locking subclasses of inode cluster lock
extern "C" {
    pub fn ocfs2_dlm_init(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_dlm_shutdown(osb: *mut ocfs2_super, hangup_pending: c_int);
}
extern "C" {
    pub fn ocfs2_lock_res_init_once(res: *mut ocfs2_lock_res);
}
extern "C" {
    pub fn ocfs2_lock_res_free(res: *mut ocfs2_lock_res);
}
extern "C" {
    pub fn ocfs2_create_new_inode_locks(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ocfs2_drop_inode_locks(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ocfs2_rw_lock(inode: *mut inode, write: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_try_rw_lock(inode: *mut inode, write: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_rw_unlock(inode: *mut inode, write: c_int);
}
extern "C" {
    pub fn ocfs2_open_lock(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ocfs2_try_open_lock(inode: *mut inode, write: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_open_unlock(inode: *mut inode);
}
// Variants without special locking class or flags

// 99% of the time we don't want to supply any additional flags --
// those are for very specific cases only.

extern "C" {
    pub fn ocfs2_orphan_scan_lock(osb: *mut ocfs2_super, seqno: *mut u32) -> c_int;
}
extern "C" {
    pub fn ocfs2_orphan_scan_unlock(osb: *mut ocfs2_super, seqno: u32);
}
extern "C" {
    pub fn ocfs2_rename_lock(osb: *mut ocfs2_super) -> c_int;
}
extern "C" {
    pub fn ocfs2_rename_unlock(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_nfs_sync_lock(osb: *mut ocfs2_super, ex: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_nfs_sync_unlock(osb: *mut ocfs2_super, ex: c_int);
}
extern "C" {
    pub fn ocfs2_trim_fs_lock_res_init(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_trim_fs_lock_res_uninit(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_dentry_lock(dentry: *mut dentry, ex: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_dentry_unlock(dentry: *mut dentry, ex: c_int);
}
extern "C" {
    pub fn ocfs2_file_lock(file: *mut file, ex: c_int, trylock: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_file_unlock(file: *mut file);
}
extern "C" {
    pub fn ocfs2_qinfo_lock(oinfo: *mut ocfs2_mem_dqinfo, ex: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_qinfo_unlock(oinfo: *mut ocfs2_mem_dqinfo, ex: c_int);
}
extern "C" {
    pub fn ocfs2_refcount_lock(ref_tree: *mut ocfs2_refcount_tree, ex: c_int) -> c_int;
}
extern "C" {
    pub fn ocfs2_refcount_unlock(ref_tree: *mut ocfs2_refcount_tree, ex: c_int);
}
// for the downconvert thread
extern "C" {
    pub fn ocfs2_wake_downconvert_thread(osb: *mut ocfs2_super);
}
extern "C" {
    pub fn ocfs2_put_dlm_debug(dlm_debug: *mut ocfs2_dlm_debug);
}
// To set the locking protocol on module initialization
extern "C" {
    pub fn ocfs2_set_locking_protocol();
}
// The _tracker pair is used to avoid cluster recursive locking
