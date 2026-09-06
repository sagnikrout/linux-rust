//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/gen_stats.h
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

// Throughput stats.
// Must be initialized beforehand with gnet_stats_basic_sync_init().
//
// If no reads can ever occur parallel to writes (e.g. stack-allocated
// bstats), then the internal stat values can be written to and read
// from directly. Otherwise, use _bstats_set/update() for writes and
// gnet_stats_add_basic() for reads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_stats_basic_sync {
    pub bytes: u64_stats_t,
    pub packets: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub sizeof(u64)): *mut *mut } __aligned(2,
    pub net_rate_estimator: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnet_dump {
    pub lock: *mut *mut spinlock_t,
    pub skb: *mut *mut sk_buff,
    pub tail: *mut *mut nlattr,
// Backward compatibility
    pub compat_tc_stats: c_int,
    pub compat_xstats: c_int,
    pub padattr: c_int,
    pub xstats: *mut *mut c_void,
    pub xstats_len: c_int,
    pub tc_stats: tc_stats,
}

extern "C" {
    pub fn gnet_stats_basic_sync_init(b: *mut gnet_stats_basic_sync);
}
extern "C" {
    pub fn gnet_stats_copy_app(d: *mut gnet_dump, st: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn gnet_stats_finish_copy(d: *mut gnet_dump) -> c_int;
}
extern "C" {
    pub fn gen_kill_estimator(ptr: *mut net_rate_estimator __rcu);
}
extern "C" {
    pub fn gen_estimator_active(ptr: *mut net_rate_estimator __rcu) -> bool;
}
