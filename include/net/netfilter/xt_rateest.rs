//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/xt_rateest.h
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
pub struct xt_rateest {
// keep lock and bstats on same cache line to speedup xt_rateest_tg()
    pub bstats: gnet_stats_basic_sync,
    pub lock: spinlock_t,
// following fields not accessed in hot path
    pub refcnt: c_uint,
    pub list: hlist_node,
    pub name: [c_char; IFNAMSIZ],
    pub params: gnet_estimator,
    pub rcu: rcu_head,
// keep this field far away to speedup xt_rateest_mt()
    pub rate_est: *mut net_rate_estimator __rcu,
}

extern "C" {
    pub fn xt_rateest_put(net: *mut net, est: *mut xt_rateest);
}
