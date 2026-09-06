//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_qos.h
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2022 Microchip Technology Inc. and its subsidiaries.
//

// Number of Layers
pub const SPX5_HSCH_LAYER_CNT: c_int = 3;
// Scheduling elements per layer
pub const SPX5_HSCH_L0_SE_CNT: c_int = 5040;
pub const SPX5_HSCH_L1_SE_CNT: c_int = 64;
pub const SPX5_HSCH_L2_SE_CNT: c_int = 64;
// Calculate Layer 0 Scheduler Element when using normal hierarchy

// Number of leak groups
pub const SPX5_HSCH_LEAK_GRP_CNT: c_int = 4;
// Scheduler modes
pub const SPX5_SE_MODE_LINERATE: c_int = 0;
pub const SPX5_SE_MODE_DATARATE: c_int = 1;
// Rate and burst
pub const SPX5_SE_RATE_MAX: c_int = 262143;
pub const SPX5_SE_BURST_MAX: c_int = 127;
pub const SPX5_SE_RATE_MIN: c_int = 1;
pub const SPX5_SE_BURST_MIN: c_int = 1;
pub const SPX5_SE_BURST_UNIT: c_int = 4096;
// Dwrr
pub const SPX5_DWRR_COST_MAX: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_shaper {
    pub mode: u32,
    pub rate: u32,
    pub burst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_lg {
    pub max_rate: u32,
    pub resolution: u32,
    pub leak_time: u32,
    pub max_ses: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_layer {
    pub leak_groups: [sparx5_lg; SPX5_HSCH_LEAK_GRP_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_dwrr {
    pub /: *mut *mut u32 count; / Number of inputs running dwrr,
    pub cost: [u8; SPX5_PRIOS],
}

extern "C" {
    pub fn sparx5_qos_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_tas_speed(port: *mut sparx5_port, speed: c_int);
}
// Multi-Queue Priority
extern "C" {
    pub fn sparx5_tc_mqprio_add(ndev: *mut net_device, num_tc: u8) -> c_int;
}
extern "C" {
    pub fn sparx5_tc_mqprio_del(ndev: *mut net_device) -> c_int;
}
// Token Bucket Filter
extern "C" {
    pub fn sparx5_tc_tbf_del(port: *mut sparx5_port, layer: u32, idx: u32) -> c_int;
}
// Enhanced Transmission Selection
extern "C" {
    pub fn sparx5_tc_ets_del(port: *mut sparx5_port) -> c_int;
}
extern "C" {
    pub fn sparx5_get_hsch_max_group_rate(grp: c_int) -> u32;
}
