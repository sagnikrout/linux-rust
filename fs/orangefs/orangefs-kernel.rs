//! Automatically rewritten from C Header to Rust Module
//! Source: fs/orangefs/orangefs-kernel.h
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
// (C) 2001 Clemson University and The University of Chicago
//
// See COPYING in top-level directory.
//
// The ORANGEFS Linux kernel support allows ORANGEFS volumes to be mounted and
// accessed through the Linux VFS (i.e. using standard I/O system calls).
// This support is only needed on clients that wish to mount the file system.
//
// Declarations and macros for the ORANGEFS Linux kernel support.
//

pub const ORANGEFS_DEFAULT_OP_TIMEOUT_SECS: c_int = 20;
pub const ORANGEFS_BUFMAP_WAIT_TIMEOUT_SECS: c_int = 30;

pub const ORANGEFS_DEVREQ_MAGIC: c_uint = 0x20030529;
pub const ORANGEFS_PURGE_RETRY_COUNT: c_uint = 0x00000005;

//
// valid orangefs kernel operation states
//
// unknown  - op was just initialized
// waiting  - op is on request_list (upward bound)
// inprogr  - op is in progress (waiting for downcall)
// serviced - op has matching downcall; ok
// purged   - op has to start a timer since client-core
// exited uncleanly before servicing op
// given up - submitter has given up waiting for it
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum orangefs_vfs_op_states {
    OP_VFS_STATE_UNKNOWN = 0,
    OP_VFS_STATE_WAITING = 1,
    OP_VFS_STATE_INPROGR = 2,
    OP_VFS_STATE_SERVICED = 4,
    OP_VFS_STATE_PURGED = 8,
    OP_VFS_STATE_GIVEN_UP = 16,
}

extern "C" {
    pub fn __orangefs_set_acl(inode: *mut inode, acl: *mut posix_acl, type: c_int) -> c_int;
}
//
// orangefs data structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_kernel_op_s {
    pub op_state: orangefs_vfs_op_states,
    pub tag: __u64,
//
// Set uses_shared_memory to non zero if this operation uses
// shared memory. If true, then a retry on the op must also
// get a new shared memory buffer and re-populate it.
// Cancels don't care - it only matters for service_operation()
// retry logics and cancels don't go through it anymore. It
// safely stays non-zero when we use it as slot_to_free.
//
    pub uses_shared_memory: c_int,
    pub slot_to_free: c_int,
}

extern "C" {
    pub fn op_release(op: *mut orangefs_kernel_op_s);
}
extern "C" {
    pub fn orangefs_bufmap_put(_arg: c_int);
}
// per inode private orangefs info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_inode_s {
    pub refn: orangefs_object_kref,
    pub link_target: [c_char; ORANGEFS_NAME_MAX],
//
// Reading/Writing Extended attributes need to acquire the appropriate
// reader/writer semaphore on the orangefs_inode_s structure.
//
    pub xattr_sem: rw_semaphore,
    pub vfs_inode: inode,
    pub last_failed_block_index_read: sector_t,
    pub getattr_time: c_ulong,
    pub mapping_time: c_ulong,
    pub attr_valid: c_int,
    pub attr_uid: kuid_t,
    pub attr_gid: kgid_t,
    pub bitlock: c_ulong,
    pub 4): DECLARE_HASHTABLE(xattr_cache,,
}

// per superblock private orangefs info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_sb_info_s {
    pub root_khandle: orangefs_khandle,
    pub fs_id: __s32,
    pub id: c_int,
    pub flags: c_int,
pub const ORANGEFS_OPT_INTR: c_uint = 0x01;
pub const ORANGEFS_OPT_LOCAL_LOCK: c_uint = 0x02;
    pub devname: [c_char; ORANGEFS_MAX_SERVER_ADDR_LEN],
    pub sb: *mut super_block,
    pub mount_pending: c_int,
    pub no_list: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_stats {
    pub cache_hits: c_ulong,
    pub cache_misses: c_ulong,
    pub reads: c_ulong,
    pub writes: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_cached_xattr {
    pub node: hlist_node,
    pub key: [c_char; ORANGEFS_MAX_XATTR_NAMELEN],
    pub val: [c_char; ORANGEFS_MAX_XATTR_VALUELEN],
    pub length: isize,
    pub timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orangefs_write_range {
    pub pos: loff_t,
    pub len: usize,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

//
// NOTE: See Documentation/filesystems/porting.rst for information
// on implementing FOO_I and properly accessing fs private data
//
extern "C" {
    pub fn container_of(_arg: inode, orangefs_inode_s: struct, _arg: vfs_inode) -> return;
}
// ino_t descends from "unsigned long", 8 bytes, 64 bits.
//
// defined in orangefs-cache.c
//
extern "C" {
    pub fn op_cache_initialize() -> c_int;
}
extern "C" {
    pub fn op_cache_finalize() -> c_int;
}
extern "C" {
    pub fn orangefs_new_tag(op: *mut orangefs_kernel_op_s);
}
extern "C" {
    pub fn orangefs_inode_cache_initialize() -> c_int;
}
extern "C" {
    pub fn orangefs_inode_cache_finalize() -> c_int;
}
//
// defined in orangefs-mod.c
//
extern "C" {
    pub fn purge_inprogress_ops();
}
//
// defined in waitqueue.c
//
extern "C" {
    pub fn purge_waiting_ops();
}
//
// defined in super.c
//
extern "C" {
    pub fn orangefs_init_fs_context(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn orangefs_kill_sb(sb: *mut super_block);
}
extern "C" {
    pub fn orangefs_remount(: *mut orangefs_sb_info_s) -> c_int;
}
extern "C" {
    pub fn fsid_key_table_initialize() -> c_int;
}
extern "C" {
    pub fn fsid_key_table_finalize();
}
//
// defined in inode.c
//
extern "C" {
    pub fn orangefs_page_mkwrite(: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn __orangefs_setattr(: *mut inode, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn __orangefs_setattr_mode(dentry: *mut dentry, iattr: *mut iattr) -> c_int;
}
extern "C" {
    pub fn orangefs_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
//
// defined in xattr.c
//
extern "C" {
    pub fn orangefs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize;
}
//
// defined in namei.c
//
// defined in devorangefs-req.c
//
extern "C" {
    pub fn orangefs_dev_init() -> c_int;
}
extern "C" {
    pub fn orangefs_dev_cleanup();
}
extern "C" {
    pub fn is_daemon_in_service() -> c_int;
}
extern "C" {
    pub fn __is_daemon_in_service() -> bool;
}
//
// defined in file.c
//
extern "C" {
    pub fn orangefs_revalidate_mapping(: *mut inode) -> c_int;
}
//
// defined in orangefs-utils.c
//
extern "C" {
    pub fn fsid_of_op(op: *mut orangefs_kernel_op_s) -> __s32;
}
pub const ORANGEFS_GETATTR_NEW: c_int = 1;
pub const ORANGEFS_GETATTR_SIZE: c_int = 2;
extern "C" {
    pub fn orangefs_inode_getattr(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn orangefs_inode_check_changed(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn orangefs_inode_setattr(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn orangefs_cancel_op_in_progress(op: *mut orangefs_kernel_op_s) -> bool;
}
extern "C" {
    pub fn orangefs_normalize_to_errno(error_code: __s32) -> c_int;
}
//
// misc convenience macros
//

pub const ORANGEFS_OP_WRITEBACK: c_int = 32;

