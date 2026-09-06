//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ptp/ptp_fc3.h
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
//
// PTP hardware clock driver for the FemtoClock3 family of timing and
// synchronization devices.
//
// Copyright (C) 2023 Integrated Device Technology, Inc., a Renesas Company.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idtfc3 {
    pub caps: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub dev: *mut device,
// Mutex to protect operations from being interrupted
    pub lock: *mut mutex,
    pub mfd: *mut device,
    pub regmap: *mut regmap,
    pub hw_param: idtfc3_hw_param,
    pub sub_sync_count: u32,
    pub ns_per_sync: u32,
    pub tdc_offset_sign: c_int,
    pub tdc_apll_freq: u64,
    pub time_ref_freq: u32,
    pub fod_n: u16,
    pub lpf_mode: u8,
// Time counter
    pub last_counter: u32,
    pub ns: i64,
    pub ns_per_counter: u32,
    pub tc_update_period: u32,
    pub tc_write_timeout: u32,
    pub tod_write_overhead: i64,
}
