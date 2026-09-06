//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/cache.h
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
// Request reply cache. This was heavily inspired by the
// implementation in 4.3BSD/4.4BSD.
//
// Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
//

//
// Representation of a reply cache entry.
//
// Note that we use a sockaddr_in6 to hold the address instead of the more
// typical sockaddr_storage. This is for space reasons, since sockaddr_storage
// is much larger than a sockaddr_in6.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfsd_cacherep {
// Keep often-read xid, csum in the same cache line:
    pub k_xid: __be32,
    pub k_csum: __wsum,
    pub k_proc: u32,
    pub k_prot: u32,
    pub k_vers: u32,
    pub k_len: c_uint,
    pub k_addr: sockaddr_in6,
    pub c_key: },
    pub c_node: rb_node,
    pub c_lru: list_head,
    pub /: *mut *mut c_secure : 1; / req came from port < 1024,
    pub c_timestamp: c_ulong,
    pub u_vec: kvec,
    pub u_status: __be32,
    pub c_u: },
}

// cache entry states
// return values
//
// Cache types.
// We may want to add more types one day, e.g. for diropres and
// attrstat replies. Using cache entries with fixed length instead
// of buffer pointers may be more efficient.
//
// Cache entries expire after this time period

// Checksum this amount of the request

extern "C" {
    pub fn nfsd_drc_slab_create() -> c_int;
}
extern "C" {
    pub fn nfsd_drc_slab_free();
}
extern "C" {
    pub fn nfsd_reply_cache_init(: *mut nfsd_net) -> c_int;
}
extern "C" {
    pub fn nfsd_reply_cache_shutdown(: *mut nfsd_net);
}
extern "C" {
    pub fn nfsd_reply_cache_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
