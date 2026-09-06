//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/meter.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2017 Nicira, Inc.
//
pub const METER_H: c_int = 1;

pub const DP_MAX_BANDS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_meter_band {
    pub type: u32,
    pub rate: u32,
    pub burst_size: u32,
    pub /: *mut *mut u64 bucket; / 1/1000 packets, or in bits,
    pub stats: ovs_flow_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_meter {
    pub /: *mut *mut spinlock_t lock; / Per meter lock,
    pub rcu: rcu_head,
    pub id: u32,
    pub keep_stats:1: u16 kbps:1,,
    pub n_bands: u16,
    pub max_delta_t: u32,
    pub used: u64,
    pub stats: ovs_flow_stats,
    pub __counted_by(n_bands): dp_meter_band bands[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_meter_instance {
    pub rcu: rcu_head,
    pub n_meters: u32,
    pub __counted_by(n_meters): *mut *mut dp_meter __rcu dp_meters[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_meter_table {
    pub ti: *mut dp_meter_instance __rcu,
    pub count: u32,
    pub max_meters_allowed: u32,
}

extern "C" {
    pub fn ovs_meters_init(dp: *mut datapath) -> c_int;
}
extern "C" {
    pub fn ovs_meters_exit(dp: *mut datapath);
}
