//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/thermal.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2020 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH12K_THERMAL_DEFAULT_DUTY_CYCLE: c_int = 100;
pub const ATH12K_THERMAL_THROTTLE_MAX: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_thermal_cfg_idx {
// Internal Power Amplifier Device
    ATH12K_TT_CFG_IDX_IPA,
// External Power Amplifier Device or External Front End Module
    ATH12K_TT_CFG_IDX_XFEM,
    ATH12K_TT_CFG_IDX_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_thermal {
    pub wmi_sync: completion,
// temperature value in Celsius degree protected by data_lock.
    pub temperature: c_int,
    pub hwmon_dev: *mut device,
    pub tt_level_configs: *const ath12k_wmi_tt_level_config_param,
    pub cdev: *mut thermal_cooling_device,
// Serialize thermal operations and hwmon reads
    pub lock: mutex,
    pub throttle_state: u32,
}

extern "C" {
    pub fn ath12k_thermal_register(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_thermal_unregister(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_thermal_event_temperature(ar: *mut ath12k, temperature: c_int);
}
extern "C" {
    pub fn ath12k_thermal_throttling_config_default(ar: *mut ath12k) -> c_int;
}
extern "C" {
    pub fn ath12k_thermal_init_configs(ar: *mut ath12k);
}
extern "C" {
    pub fn ath12k_thermal_set_throttling(ar: *mut ath12k, throttle_state: u32) -> c_int;
}

