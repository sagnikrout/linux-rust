//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/ptp.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2025 Intel Corporation
//

// Macro flag: #define __iwl_mld_ptp_h__

//
// struct ptp_data - PTP hardware clock data
//
// @ptp_clock: struct ptp_clock pointer returned by the ptp_clock_register()
// function.
// @ptp_clock_info: struct ptp_clock_info that describes a PTP hardware clock
// @lock: protects the time adjustments data
// @delta: delta between hardware clock and ptp clock in nanoseconds
// @scale_update_gp2: GP2 time when the scale was last updated
// @scale_update_adj_time_ns: adjusted time when the scale was last updated,
// in nanoseconds
// @scaled_freq: clock frequency offset, scaled to 65536000000
// @last_gp2: the last GP2 reading from the hardware, used for tracking GP2
// wraparounds
// @wrap_counter: number of wraparounds since scale_update_adj_time_ns
// @dwork: worker scheduled every 1 hour to detect workarounds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_data {
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
    pub lock: spinlock_t,
    pub delta: i64,
    pub scale_update_gp2: u32,
    pub scale_update_adj_time_ns: u64,
    pub scaled_freq: u64,
    pub last_gp2: u32,
    pub wrap_counter: u32,
    pub dwork: delayed_work,
}

extern "C" {
    pub fn iwl_mld_ptp_init(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_ptp_remove(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_ptp_get_adj_time(mld: *mut iwl_mld, base_time_ns: u64) -> u64;
}
