//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/export.h
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
// Copyright (C) 1995-1997 Olaf Kirch <okir@monad.swb.de>
//

//
// FS Locations
//
pub const MAX_FS_LOCATIONS: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_fs_location {
    pub /: *mut *mut *mut char hosts; / colon separated list of hosts,
    pub /: *mut *mut *mut char path; / slash separated list of path components,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd4_fs_locations {
    pub locations_count: u32,
    pub locations: *mut nfsd4_fs_location,
// If we're not actually serving this data ourselves (only providing a
// list of replicas that do serve it) then we set "migrated":
    pub migrated: c_int,
}

//
// We keep an array of pseudoflavors with the export, in order from most
// to least preferred.  For the foreseeable future, we don't expect more
// than the eight pseudoflavors null, unix, krb5, krb5i, krb5p, skpm3,
// spkm3i, and spkm3p (and using all 8 at once should be rare).
//
pub const MAX_SECINFO_LIST: c_int = 8;
pub const EX_UUID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exp_flavor_info {
    pub pseudoflavor: u32,
    pub flags: u32,
}

// Per-export stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct export_stats {
    pub start_time: time64_t,
    pub counter: [percpu_counter; EXP_STATS_COUNTERS_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_export {
    pub h: cache_head,
    pub ex_client: *mut *mut auth_domain,
    pub ex_flags: c_int,
    pub ex_fsid: c_int,
    pub ex_path: path,
    pub ex_anon_uid: kuid_t,
    pub ex_anon_gid: kgid_t,
    pub /: *mut *mut *mut unsigned char  ex_uuid; / 16 byte fsid,
    pub ex_fslocs: nfsd4_fs_locations,
    pub ex_nflavors: u32,
    pub ex_flavors: [exp_flavor_info; MAX_SECINFO_LIST],
    pub ex_layout_types: u32,
    pub ex_devid_map: *mut nfsd4_deviceid_map,
    pub cd: *mut cache_detail,
    pub ex_rcu: rcu_head,
    pub ex_xprtsec_modes: c_ulong,
    pub ex_stats: *mut export_stats,
}

// an "export key" (expkey) maps a filehandlefragement to an
// svc_export for a given client.  There can be several per export,
// for the different fsid types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_expkey {
    pub h: cache_head,
    pub ek_client: *mut *mut auth_domain,
    pub ek_fsidtype: u8,
    pub ek_fsid: [u32; 6],
    pub ek_path: path,
    pub ek_rcu: rcu_head,
}

extern "C" {
    pub fn nfsexp_flags(cred: *mut svc_cred, exp: *mut svc_export) -> c_int;
}
extern "C" {
    pub fn check_xprtsec_policy(exp: *mut svc_export, rqstp: *mut svc_rqst) -> __be32;
}
//
// Function declarations
//
extern "C" {
    pub fn nfsd_export_init(: *mut net) -> c_int;
}
extern "C" {
    pub fn nfsd_export_shutdown(: *mut net);
}
extern "C" {
    pub fn nfsd_export_flush(: *mut net);
}
extern "C" {
    pub fn rqst_find_fsidzero_export(: *mut svc_rqst) -> *mut svc_export;
}
extern "C" {
    pub fn exp_pseudoroot(: *mut svc_rqst, : *mut svc_fh) -> __be32;
}
