//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/net_failover.h
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
// Copyright (c) 2018, Intel Corporation.

// failover state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_failover_info {
// primary netdev with same MAC
    pub primary_dev: *mut net_device __rcu,
// standby netdev
    pub standby_dev: *mut net_device __rcu,
// primary netdev stats
    pub primary_stats: rtnl_link_stats64,
// standby netdev stats
    pub standby_stats: rtnl_link_stats64,
// aggregated stats
    pub failover_stats: rtnl_link_stats64,
// spinlock while updating stats
    pub stats_lock: spinlock_t,
}

extern "C" {
    pub fn net_failover_destroy(failover: *mut failover);
}

