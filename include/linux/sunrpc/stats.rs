//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/stats.h
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
// linux/include/linux/sunrpc/stats.h
//
// Client statistics collection for SUN RPC
//
// Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_stat {
    pub program: *const rpc_program,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_stat {
    pub program: *mut *mut svc_program,
// Per-version per-procedure call counts (per-cpu, per-netns)
    pub vs_count: *mut unsigned long __percpu,
}

extern "C" {
    pub fn svc_stat_alloc_counts(statp: *mut svc_stat) -> c_int;
}
extern "C" {
    pub fn svc_stat_free_counts(statp: *mut svc_stat);
}

extern "C" {
    pub fn rpc_proc_init(: *mut net) -> c_int;
}
extern "C" {
    pub fn rpc_proc_exit(: *mut net);
}
extern "C" {
    pub fn rpc_proc_register(: *mut net, : *mut rpc_stat) -> *mut proc_dir_entry;
}
extern "C" {
    pub fn rpc_proc_unregister(: *mut net, : *const c_char);
}
extern "C" {
    pub fn rpc_proc_zero(: *const rpc_program);
}
extern "C" {
    pub fn svc_proc_unregister(: *mut net, : *const c_char);
}

