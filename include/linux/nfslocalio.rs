//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfslocalio.h
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
// Copyright (C) 2024 Mike Snitzer <snitzer@hammerspace.com>
// Copyright (C) 2024 NeilBrown <neilb@suse.de>
//

//
// Useful to allow a client to negotiate if localio
// possible with its server.
//
// See Documentation/filesystems/nfs/localio.rst for more detail.
//
// this struct is over a cacheline, avoid bouncing
// Local files to close when net is shut down or exports change
extern "C" {
    pub fn nfs_uuid_init(: *mut nfs_uuid_t);
}
extern "C" {
    pub fn nfs_uuid_begin(: *mut nfs_uuid_t) -> bool;
}
extern "C" {
    pub fn nfs_uuid_end(: *mut nfs_uuid_t);
}
extern "C" {
    pub fn nfs_localio_enable_client(clp: *mut nfs_client);
}
extern "C" {
    pub fn nfs_localio_disable_client(clp: *mut nfs_client);
}
// localio needs to map filehandle -> struct nfsd_file
extern "C" {
    pub fn nfs_close_local_fh(: *mut nfs_file_localio);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_localio_operations {
    pub ): *mut *mut bool (nfsd_net_try_get)(struct net,
    pub ): *mut *mut void (nfsd_net_put)(struct net,
    pub fmode_t): const,
    pub ): *mut *mut *mut net (nfsd_file_put_local)(nfsd_file __rcu,
    pub ): *mut *mut *mut file (nfsd_file_file)(nfsd_file,
    pub ): *mut *mut *mut u32 , u32 , u32,
    pub ____cacheline_aligned: },
    pub nfsd_localio_ops_init(void): extern void,
    pub nfs_to: *const extern struct nfsd_localio_operations,
    pub fmode_t): const,
//
// Once reference to net (and associated nfsd_serv) is dropped, NFSD
// could be unloaded, so ensure safe return from nfsd_net_put() by
// always taking RCU.
//
// Either *localio must be guaranteed to be non-NULL, or caller
// must prevent nfsd shutdown from completing as nfs_close_local_fh()
// does by blocking the nfs_uuid from being finally put.
//
    pub net: *mut net,
    pub nfs_to->nfsd_file_put_local(localio): net =,

    pub nfs_file_localio: struct,
    pub nfs_client: struct,

