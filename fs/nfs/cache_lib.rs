//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/cache_lib.h
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
// Helper routines for the NFS client caches
//
// Copyright (c) 2009 Trond Myklebust <Trond.Myklebust@netapp.com>
//

//
// Deferred request handling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_cache_defer_req {
    pub req: cache_req,
    pub deferred_req: cache_deferred_req,
    pub completion: completion,
    pub count: refcount_t,
}

extern "C" {
    pub fn nfs_cache_upcall(cd: *mut cache_detail, entry_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn nfs_cache_defer_req_put(dreq: *mut nfs_cache_defer_req);
}
extern "C" {
    pub fn nfs_cache_wait_for_upcall(dreq: *mut nfs_cache_defer_req) -> c_int;
}
extern "C" {
    pub fn nfs_cache_register_net(net: *mut net, cd: *mut cache_detail) -> c_int;
}
extern "C" {
    pub fn nfs_cache_unregister_net(net: *mut net, cd: *mut cache_detail);
}
