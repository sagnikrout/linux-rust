//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_hw_btrs.h
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
// Copyright (C) 2020-2026 Intel Corporation
//

pub const PLL_PROFILING_FREQ_DEFAULT: c_int = 38400000;
pub const PLL_PROFILING_FREQ_HIGH: c_int = 400000000;

extern "C" {
    pub fn ivpu_hw_btrs_info_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_freq_ratios_init(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_irqs_clear_with_0_mtl(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_wp_drive(vdev: *mut ivpu_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_wait_for_clock_res_own_ack(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_d0i3_enable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_d0i3_disable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_set_port_arbitration_weights_lnl(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_is_idle(vdev: *mut ivpu_device) -> bool;
}
extern "C" {
    pub fn ivpu_hw_btrs_wait_for_idle(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_ip_reset(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_profiling_freq_reg_set_lnl(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_ats_print_lnl(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_clock_relinquish_disable_lnl(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_pll_ratio_to_mhz(vdev: *mut ivpu_device, pll_ratio: u8) -> u32;
}
extern "C" {
    pub fn ivpu_hw_btrs_pll_ratio_to_hz(vdev: *mut ivpu_device, pll_ratio: u8) -> u32;
}
extern "C" {
    pub fn ivpu_hw_btrs_current_freq_get(vdev: *mut ivpu_device) -> u32;
}
extern "C" {
    pub fn ivpu_hw_btrs_cfg_min_freq_set(vdev: *mut ivpu_device, freq_mhz: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_cfg_max_freq_set(vdev: *mut ivpu_device, freq_mhz: u32) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_cfg_freq_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_irq_handler_mtl(vdev: *mut ivpu_device, irq: c_int) -> bool;
}
extern "C" {
    pub fn ivpu_hw_btrs_irq_handler_lnl(vdev: *mut ivpu_device, irq: c_int) -> bool;
}
extern "C" {
    pub fn ivpu_hw_btrs_dct_get_request(vdev: *mut ivpu_device, enable: *mut bool) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_btrs_dct_set_status(vdev: *mut ivpu_device, enable: bool, active_percent: u8);
}
extern "C" {
    pub fn ivpu_hw_btrs_telemetry_offset_get(vdev: *mut ivpu_device) -> u32;
}
extern "C" {
    pub fn ivpu_hw_btrs_telemetry_size_get(vdev: *mut ivpu_device) -> u32;
}
extern "C" {
    pub fn ivpu_hw_btrs_telemetry_enable_get(vdev: *mut ivpu_device) -> u32;
}
extern "C" {
    pub fn ivpu_hw_btrs_global_int_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_global_int_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_irq_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_irq_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_diagnose_failure(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_btrs_platform_read(vdev: *mut ivpu_device) -> c_int;
}
