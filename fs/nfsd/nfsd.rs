//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/nfsd.h
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
// Hodge-podge collection of knfsd-related stuff.
// I will sort this out later.
//
// Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
//

//
// nfsd version
//
pub const NFSD_MINVERS: c_int = 2;
pub const NFSD_MAXVERS: c_int = 4;
pub const NFSD_SUPPORTED_MINOR_VERSION: c_int = 2;
extern "C" {
    pub fn nfsd_support_version(vers: c_int) -> bool;
}
//
// Default and maximum payload size (NFS READ or WRITE), in bytes.
// The maximum is an implementation limit.
//
// Maximum number of operations per session compound
pub const NFSD_MAX_OPS_PER_COMPOUND: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_thread_local_info {
    pub ntli_lease_breaker: *mut nfs4_client,
    pub ntli_cachetype: c_int,
}

//
// Common void argument and result helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_voidargs {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_voidres {
    pub xdr): *mut xdr_stream,
    pub xdr): *mut xdr_stream,
//
// Function prototypes.
//
    pub scope): *const *const cred cred, char,
    pub rqstp): *mut int nfsd_dispatch(struct svc_rqst,
    pub ): *mut int nfsd_nrthreads(struct net,
    pub ): *mut int nfsd_nrpools(struct net,
    pub ): *mut *mut int nfsd_get_nrthreads(int n, int , struct net,
    pub ): *mut *mut int nfsd_set_nrthreads(int n, int , struct net,
    pub net): *mut void nfsd_shutdown_threads(struct net,
    pub nfsd_current_rqst(void): *mut svc_rqst,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsdfs_client {
    pub cl_ref: kref,
    pub kref): *mut *mut void (cl_release)(struct kref,
}

extern "C" {
    pub fn nfsd_client_rmdir(dentry: *mut dentry);
}
extern "C" {
    pub fn nfsd_cache_notify(cd: *mut cache_detail, h: *mut cache_head, cache_type: u32) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vers_op {
    int nfsd_vers(struct nfsd_net *nn, int vers, enum vers_op change);
    int nfsd_minorversion(struct nfsd_net *nn, u32 minorversion, enum vers_op change);
    void nfsd_reset_versions(struct nfsd_net *nn);
    int nfsd_create_serv(struct net *net);
    void nfsd_destroy_serv(struct net *net);

    void nfsd_debugfs_init(void);
    void nfsd_debugfs_exit(void);

    static inline void nfsd_debugfs_init(void) {}
    static inline void nfsd_debugfs_exit(void) {}

    extern bool nfsd_disable_splice_read __read_mostly;
    extern bool nfsd_delegts_enabled __read_mostly;

    enum {
// Any new NFSD_IO enum value must be added at the end
    NFSD_IO_BUFFERED,
    NFSD_IO_DONTCACHE,
    NFSD_IO_DIRECT,
}

//
// NFSv4 State
//

extern "C" {
    pub fn nfsd4_init_slabs() -> c_int;
}
extern "C" {
    pub fn nfsd4_free_slabs();
}
extern "C" {
    pub fn nfs4_state_start() -> c_int;
}
extern "C" {
    pub fn nfs4_state_start_net(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nfs4_state_shutdown();
}
extern "C" {
    pub fn nfs4_state_shutdown_net(net: *mut net);
}
extern "C" {
    pub fn nfs4_reset_recoverydir(recdir: *mut c_char) -> c_int;
}
extern "C" {
    pub fn nfs4_recoverydir() -> *mut c_char;
}
extern "C" {
    pub fn nfsd4_spo_must_allow(rqstp: *mut svc_rqst) -> bool;
}
extern "C" {
    pub fn nfsd4_create_laundry_wq() -> c_int;
}
extern "C" {
    pub fn nfsd4_destroy_laundry_wq();
}
extern "C" {
    pub fn nfsd_wait_for_delegreturn(rqstp: *mut svc_rqst, inode: *mut inode) -> bool;
}

//
// lockd binding
//
extern "C" {
    pub fn nfsd_lockd_init();
}
extern "C" {
    pub fn nfsd_lockd_shutdown();
}
//
// These macros provide pre-xdr'ed values for faster operation.
//

//
// Error codes for internal use.  These are based at an impossible
// nfsstat4 value so that, once converted to be32, they cannot conflict
// with any value defined by the protocol (compare the nlm__int__* codes
// in fs/lockd/lockd.h).
//
// end-of-file indicator in readdir

// replay detected

// nfs41 replay detected

// symlink found where dir expected - handled differently to
// other symlink found errors by NFSv3.
//

// before processing a COMPOUND operation, we have to check that there
// is enough space in the buffer for XDR encode to succeed.  otherwise,
// we might process an operation with side effects, and be unable to
// tell the client that the operation succeeded.
//
// COMPOUND_SLACK_SPACE - this is the minimum bytes of buffer space
// needed to encode an "ordinary" _successful_ operation.  (GETATTR,
// READ, READDIR, and READLINK have their own buffer checks.)  if we
// fall below this level, we fail the next operation with NFS4ERR_RESOURCE.
//
// COMPOUND_ERR_SLACK_SPACE - this is the minimum bytes of buffer space
// needed to encode an operation which has failed with NFS4ERR_RESOURCE.
// care is taken to ensure that we never fall below this level for any
// reason.
//

pub const NFSD_CLIENT_MAX_TRIM_PER_RUN: c_int = 128;
pub const NFS4_CLIENTS_PER_GB: c_int = 1024;

extern "C" {
    pub fn nfsd4_is_junction(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn register_cld_notifier() -> c_int;
}
extern "C" {
    pub fn unregister_cld_notifier();
}

extern "C" {
    pub fn nfsd4_ssc_init_umount_work(nn: *mut nfsd_net);
}

extern "C" {
    pub fn nfsd4_init_leases_net(nn: *mut nfsd_net);
}

pub const register_cld_notifier(): c_int = 0;

