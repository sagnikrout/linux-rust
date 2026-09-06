//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/delegation.h
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
// linux/fs/nfs/delegation.h
//
// Copyright (c) Trond Myklebust
//
// Definitions pertaining to NFS delegated files
//

//
// NFSv4 delegation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_delegation {
    pub hash: hlist_node,
    pub super_list: list_head,
    pub cred: *const cred,
    pub inode: *mut inode,
    pub stateid: nfs4_stateid,
    pub type: fmode_t,
    pub pagemod_limit: c_ulong,
    pub change_attr: __u64,
    pub test_gen: c_ulong,
    pub flags: c_ulong,
    pub refcount: refcount_t,
    pub lock: spinlock_t,
    pub entry: list_head,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn nfs4_inode_return_delegation(inode: *mut inode);
}
extern "C" {
    pub fn nfs4_inode_return_delegation_on_close(inode: *mut inode);
}
extern "C" {
    pub fn nfs4_inode_set_return_delegation_on_close(inode: *mut inode);
}
extern "C" {
    pub fn nfs_async_inode_return_delegation(inode: *mut inode, stateid: *const nfs4_stateid) -> c_int;
}
extern "C" {
    pub fn nfs_inode_evict_delegation(inode: *mut inode);
}
extern "C" {
    pub fn nfs_server_return_all_delegations(: *mut nfs_server);
}
extern "C" {
    pub fn nfs_expire_all_delegations(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_expire_unused_delegation_types(clp: *mut nfs_client, flags: fmode_t);
}
extern "C" {
    pub fn nfs_expire_unreferenced_delegations(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_client_return_marked_delegations(clp: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs_delegations_present(clp: *mut nfs_client) -> c_int;
}
extern "C" {
    pub fn nfs_remove_bad_delegation(inode: *mut inode, stateid: *const nfs4_stateid);
}
extern "C" {
    pub fn nfs_delegation_mark_returned(inode: *mut inode, stateid: *const nfs4_stateid);
}
extern "C" {
    pub fn nfs_delegation_mark_reclaim(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_delegation_reap_unclaimed(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_mark_test_expired_all_delegations(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_test_expired_all_delegations(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_reap_expired_delegations(clp: *mut nfs_client);
}
// NFSv4 delegation-related procedures
extern "C" {
    pub fn nfs4_open_delegation_recall(ctx: *mut nfs_open_context, state: *mut nfs4_state, stateid: *const nfs4_stateid) -> c_int;
}
extern "C" {
    pub fn nfs4_lock_delegation_recall(fl: *mut file_lock, state: *mut nfs4_state, stateid: *const nfs4_stateid) -> c_int;
}
extern "C" {
    pub fn nfs4_copy_delegation_stateid(inode: *mut inode, flags: fmode_t, dst: *mut nfs4_stateid, cred: *const cred) -> bool;
}
extern "C" {
    pub fn nfs4_refresh_delegation_stateid(dst: *mut nfs4_stateid, inode: *mut inode) -> bool;
}
extern "C" {
    pub fn nfs_put_delegation(delegation: *mut nfs_delegation);
}
extern "C" {
    pub fn nfs_mark_delegation_referenced(delegation: *mut nfs_delegation);
}
extern "C" {
    pub fn nfs4_have_delegation(inode: *mut inode, type: fmode_t, flags: c_int) -> c_int;
}
extern "C" {
    pub fn nfs4_check_delegation(inode: *mut inode, type: fmode_t) -> c_int;
}
extern "C" {
    pub fn nfs4_delegation_flush_on_close(inode: *const inode) -> bool;
}
extern "C" {
    pub fn nfs4_inode_make_writeable(inode: *mut inode);
}

extern "C" {
    pub fn nfs_update_delegated_atime(inode: *mut inode);
}
extern "C" {
    pub fn nfs_update_delegated_mtime(inode: *mut inode);
}
extern "C" {
    pub fn nfs_update_delegated_mtime_locked(inode: *mut inode);
}
extern "C" {
    pub fn NFS_PROTO(_arg: inode)->have_delegation(inode, _arg: FMODE_READ, _arg: 0) -> return;
}
extern "C" {
    pub fn NFS_PROTO(_arg: inode)->have_delegation(inode, _arg: FMODE_WRITE, _arg: 0) -> return;
}
extern "C" {
    pub fn NFS_PROTO(_arg: inode)->have_delegation(inode, _arg: FMODE_READ, _arg: 0) -> return;
}
extern "C" {
    pub fn S_ISDIR(nfs_have_delegated_attributes(inode: inode->i_mode) &&) -> return;
}
extern "C" {
    pub fn nfs4_delegation_hash_alloc(server: *mut nfs_server) -> c_int;
}
