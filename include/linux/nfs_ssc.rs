//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs_ssc.h
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
// include/linux/nfs_ssc.h
//
// Author: Dai Ngo <dai.ngo@oracle.com>
//
// Copyright (c) 2020, Oracle and/or its affiliates.
//

//
// NFS_V4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs4_ssc_client_ops {
    pub stateid): *mut *mut nfs_fh src_fh, nfs4_stateid,
    pub filep): *mut *mut void (sco_close)(struct file,
}

//
// NFS_FS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_ssc_client_ops {
    pub sb): *mut *mut void (sco_sb_deactive)(struct super_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_ssc_client_ops_tbl {
    pub ssc_nfs4_ops: *const nfs4_ssc_client_ops,
    pub ssc_nfs_ops: *const nfs_ssc_client_ops,
}

extern "C" {
    pub fn nfs42_ssc_register_ops();
}
extern "C" {
    pub fn nfs42_ssc_unregister_ops();
}
extern "C" {
    pub fn nfs42_ssc_register(ops: *const nfs4_ssc_client_ops);
}
extern "C" {
    pub fn nfs42_ssc_unregister(ops: *const nfs4_ssc_client_ops);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EIO) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_ssc_umount_item {
    pub nsui_list: list_head,
    pub nsui_busy: bool,
//
// nsui_refcnt inited to 2, 1 on list and 1 for consumer. Entry
// is removed when refcnt drops to 1 and nsui_expire expires.
//
    pub nsui_refcnt: refcount_t,
    pub nsui_expire: c_ulong,
    pub nsui_vfsmount: *mut vfsmount,
    pub 1]: char nsui_ipaddr[RPC_MAX_ADDRBUFLEN +,
}

//
// NFS_FS
//
extern "C" {
    pub fn nfs_ssc_register(ops: *const nfs_ssc_client_ops);
}
extern "C" {
    pub fn nfs_ssc_unregister(ops: *const nfs_ssc_client_ops);
}
