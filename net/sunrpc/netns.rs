//! Automatically rewritten from C Header to Rust Module
//! Source: net/sunrpc/netns.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunrpc_net {
    pub proc_net_rpc: *mut proc_dir_entry,
    pub ip_map_cache: *mut cache_detail,
    pub unix_gid_cache: *mut cache_detail,
    pub rsc_cache: *mut cache_detail,
    pub rsi_cache: *mut cache_detail,
    pub pipefs_sb: *mut super_block,
    pub gssd_dummy: *mut rpc_pipe,
    pub pipefs_sb_lock: mutex,
    pub all_clients: list_head,
    pub rpc_client_lock: spinlock_t,
    pub rpcb_local_clnt: *mut rpc_clnt,
    pub rpcb_local_clnt4: *mut rpc_clnt,
    pub rpcb_clnt_lock: spinlock_t,
    pub rpcb_users: c_uint,
    pub 1: unsigned int rpcb_is_af_local :,
    pub gssp_lock: mutex,
    pub gssp_clnt: *mut rpc_clnt,
    pub use_gss_proxy: c_int,
    pub pipe_version: c_int,
    pub pipe_users: core::sync::atomic::AtomicI32,
    pub use_gssp_proc: *mut proc_dir_entry,
    pub gss_krb5_enctypes: *mut proc_dir_entry,
}

extern "C" {
    pub fn ip_map_cache_create(: *mut net) -> c_int;
}
extern "C" {
    pub fn ip_map_cache_destroy(: *mut net);
}
