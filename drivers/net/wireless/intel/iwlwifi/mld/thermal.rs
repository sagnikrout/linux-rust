//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/thermal.h
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
// Copyright (C) 2024 Intel Corporation
//

// Macro flag: #define __iwl_mld_thermal_h__

//
// struct iwl_mld_cooling_device
// @cur_state: current state
// @cdev: struct thermal cooling device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_cooling_device {
    pub cur_state: u32,
    pub cdev: *mut thermal_cooling_device,
}

extern "C" {
    pub fn iwl_mld_handle_temp_notif(mld: *mut iwl_mld, pkt: *mut iwl_rx_packet);
}
extern "C" {
    pub fn iwl_mld_config_temp_report_ths(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_thermal_initialize(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_thermal_exit(mld: *mut iwl_mld);
}
