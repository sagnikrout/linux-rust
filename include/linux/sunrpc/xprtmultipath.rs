//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/xprtmultipath.h
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
// RPC client multipathing definitions
//
// Copyright (c) 2015, 2016, Primary Data, Inc. All rights reserved.
//
// Trond Myklebust <trond.myklebust@primarydata.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_xprt_switch {
    pub xps_lock: spinlock_t,
    pub xps_kref: kref,
    pub xps_id: c_uint,
    pub xps_nxprts: c_uint,
    pub xps_nactive: c_uint,
    pub xps_nunique_destaddr_xprts: c_uint,
    pub xps_queuelen: atomic_long_t,
    pub xps_xprt_list: list_head,
    pub xps_net: *mut *mut net,
    pub xps_iter_ops: *const rpc_xprt_iter_ops,
    pub xps_sysfs: *mut rpc_sysfs_xprt_switch,
    pub xps_rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_xprt_iter {
    pub xpi_xpswitch: *mut rpc_xprt_switch __rcu,
    pub xpi_cursor: *mut *mut rpc_xprt,
    pub xpi_ops: *const rpc_xprt_iter_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_xprt_iter_ops {
    pub ): *mut *mut void (xpi_rewind)(struct rpc_xprt_iter,
    pub ): *mut *mut *mut rpc_xprt (xpi_xprt)(rpc_xprt_iter,
    pub ): *mut *mut *mut rpc_xprt (xpi_next)(rpc_xprt_iter,
}

extern "C" {
    pub fn xprt_switch_put(xps: *mut rpc_xprt_switch);
}
extern "C" {
    pub fn rpc_xprt_switch_set_roundrobin(xps: *mut rpc_xprt_switch);
}
extern "C" {
    pub fn xprt_iter_destroy(xpi: *mut rpc_xprt_iter);
}
extern "C" {
    pub fn xprt_iter_rewind(xpi: *mut rpc_xprt_iter);
}
extern "C" {
    pub fn xprt_multipath_cleanup_ids();
}
