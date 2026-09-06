//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/vfs_cache.h
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
// Copyright (C) 2019 Samsung Electronics Co., Ltd.
//

// Windows style file permissions for extended response
pub const FILE_GENERIC_ALL: c_uint = 0x1F01FF;
pub const FILE_GENERIC_READ: c_uint = 0x120089;
pub const FILE_GENERIC_WRITE: c_uint = 0x120116;

//
// Start volatile/persistent file id allocation at 1. A file id of 0 yields an
// SMB2 FileId of {0, 0}, which clients (e.g. Windows, Samba) treat as a null
// handle and never close, leaking the open on the server.
//
pub const KSMBD_START_FID: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_lock {
    pub fl: *mut file_lock,
    pub conn: *mut ksmbd_conn,
    pub clist: list_head,
    pub flist: list_head,
    pub llist: list_head,
    pub flags: c_uint,
    pub cmd: c_int,
    pub zero_len: c_int,
    pub start: c_ulonglong,
    pub end: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream {
    pub name: *mut c_char,
    pub size: isize,
    pub pos: loff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_inode {
    pub m_lock: rw_semaphore,
    pub m_count: core::sync::atomic::AtomicI32,
    pub op_count: core::sync::atomic::AtomicI32,
// opinfo count for streams
    pub sop_count: core::sync::atomic::AtomicI32,
    pub m_de: *mut dentry,
    pub m_flags: c_uint,
    pub m_hash: hlist_node,
    pub m_fp_list: list_head,
    pub m_op_list: list_head,
    pub m_opinfo: *mut oplock_info,
    pub m_fattr: __le32,
}

// Owner information for durable handle reconnect
#[repr(C)]
#[derive(Copy, Clone)]
pub struct durable_owner {
    pub uid: c_uint,
    pub gid: c_uint,
    pub name: *mut c_char,
}

pub const KSMBD_LOCK_SEQ_ARRAY_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_lock_sequence {
    pub valid: bool,
    pub sequence: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_file {
    pub filp: *mut file,
    pub persistent_id: u64,
    pub volatile_id: u64,
    pub durable_volatile_id: u64,
    pub f_lock: spinlock_t,
    pub f_ci: *mut ksmbd_inode,
    pub f_parent_ci: *mut ksmbd_inode,
    pub f_opinfo: *mut oplock_info __rcu,
    pub conn: *mut ksmbd_conn,
    pub tcon: *mut ksmbd_tree_connect,
    pub refcount: core::sync::atomic::AtomicI32,
    pub daccess: __le32,
    pub saccess: __le32,
    pub coption: __le32,
    pub cdoption: __le32,
    pub create_file_attributes: __le32,
    pub create_time: __u64,
    pub change_time: __u64,
    pub allocation_size: __u64,
    pub itime: __u64,
    pub open_mtime: __u64,
    pub is_nt_open: bool,
    pub attrib_only: bool,
    pub allocation_size_set: bool,
    pub client_guid: [c_char; 16],
    pub create_guid: [c_char; 16],
    pub app_instance_id: [c_char; 16],
    pub stream: stream,
    pub node: list_head,
    pub blocked_works: list_head,
    pub lock_list: list_head,
//
// Per-handle FileDispositionInformation delete-pending state for a
// stream handle -- separate from ksmbd_inode's inode-wide m_flags,
// which have no way to record which stream on a multi-stream file
// was actually marked for deletion. See ksmbd_fd_set_delete_pending().
//
    pub stream_del_pending: bool,
    pub durable_timeout: c_uint,
    pub durable_scavenger_timeout: c_uint,
// CREATE action returned when this durable handle was established.
    pub create_action: __le32,
// if ls is happening on directory, below is valid
    pub readdir_data: ksmbd_readdir_data,
    pub readdir_lock: mutex,
    pub dot_dotdot: [c_int; 2],
    pub f_state: c_uint,
    pub reserve_lease_break: bool,
    pub is_durable: bool,
    pub is_persistent: bool,
    pub is_resilient: bool,
    pub has_app_instance_id: bool,
    pub app_instance_version_valid: bool,
    pub app_instance_version_high: u64,
    pub app_instance_version_low: u64,
    pub durable_reconnect_disabled: bool,
    pub durable_replay_consumed: bool,
    pub is_posix_ctxt: bool,
    pub owner: durable_owner,
    pub channel_sequence: __le16,
    pub outstanding_requests: c_uint,
    pub outstanding_pre_requests: c_uint,
    pub lock_seq: [ksmbd_lock_sequence; KSMBD_LOCK_SEQ_ARRAY_SIZE],
//
// Pending CHANGE_NOTIFY completions for this handle, sent with
// STATUS_NOTIFY_CLEANUP when the handle is closed.
//
    pub notify_pendings: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_file_table {
    pub lock: rwlock_t,
    pub idr: *mut idr,
}

extern "C" {
    pub fn ksmbd_init_file_table(ft: *mut ksmbd_file_table) -> c_int;
}
extern "C" {
    pub fn ksmbd_destroy_file_table(sess: *mut ksmbd_session);
}
extern "C" {
    pub fn ksmbd_close_fd(work: *mut ksmbd_work, id: u64) -> c_int;
}
extern "C" {
    pub fn ksmbd_fd_put(work: *mut ksmbd_work, fp: *mut ksmbd_file);
}
extern "C" {
    pub fn ksmbd_inode_put(ci: *mut ksmbd_inode);
}
extern "C" {
    pub fn ksmbd_close_disconnected_durable_delete_on_close(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ksmbd_put_durable_fd(fp: *mut ksmbd_file);
}
extern "C" {
    pub fn ksmbd_invalidate_durable_fd(id: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn ksmbd_has_other_active_fd(fp: *mut ksmbd_file) -> bool;
}
extern "C" {
    pub fn ksmbd_has_stream_without_delete_share(fp: *mut ksmbd_file) -> bool;
}
extern "C" {
    pub fn ksmbd_close_fd_app_instance_id(app_instance_id: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ksmbd_has_other_nonposix_open(dentry: *mut dentry) -> bool;
}
extern "C" {
    pub fn ksmbd_has_nonposix_open_child(old_fp: *mut ksmbd_file) -> bool;
}
extern "C" {
    pub fn ksmbd_open_durable_fd(fp: *mut ksmbd_file) -> c_uint;
}
extern "C" {
    pub fn ksmbd_launch_ksmbd_durable_scavenger();
}
extern "C" {
    pub fn ksmbd_stop_durable_scavenger();
}
extern "C" {
    pub fn ksmbd_durable_scavenger_active() -> bool;
}
extern "C" {
    pub fn ksmbd_close_tree_conn_fds(work: *mut ksmbd_work);
}
extern "C" {
    pub fn ksmbd_close_session_fds(work: *mut ksmbd_work);
}
extern "C" {
    pub fn ksmbd_wake_session_blocked_works(sess: *mut ksmbd_session);
}
extern "C" {
    pub fn ksmbd_close_inode_fds(work: *mut ksmbd_work, inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn ksmbd_init_global_file_table() -> c_int;
}
extern "C" {
    pub fn ksmbd_free_global_file_table();
}
extern "C" {
    pub fn ksmbd_set_fd_limit(limit: c_ulong);
}
//
// INODE hash
//
extern "C" {
    pub fn ksmbd_inode_hash_init() -> int __init;
}
extern "C" {
    pub fn ksmbd_release_inode_hash();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KSMBD_INODE_STATUS {
    KSMBD_INODE_STATUS_OK,
    KSMBD_INODE_STATUS_UNKNOWN,
    KSMBD_INODE_STATUS_PENDING_DELETE,
}

extern "C" {
    pub fn ksmbd_query_inode_status(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ksmbd_inode_pending_delete(fp: *mut ksmbd_file) -> bool;
}
extern "C" {
    pub fn ksmbd_set_inode_pending_delete(fp: *mut ksmbd_file);
}
extern "C" {
    pub fn ksmbd_clear_inode_pending_delete(fp: *mut ksmbd_file);
}
extern "C" {
    pub fn ksmbd_fd_set_delete_pending(fp: *mut ksmbd_file);
}
extern "C" {
    pub fn ksmbd_fd_clear_delete_pending(fp: *mut ksmbd_file);
}
extern "C" {
    pub fn ksmbd_reopen_durable_fd(work: *mut ksmbd_work, fp: *mut ksmbd_file) -> c_int;
}
extern "C" {
    pub fn ksmbd_init_file_cache() -> c_int;
}
extern "C" {
    pub fn ksmbd_exit_file_cache();
}
