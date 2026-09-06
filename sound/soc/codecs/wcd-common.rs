//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wcd-common.h
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
// Copyright (c) 2025, Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const WCD_MAX_MICBIAS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_sdw_ch_info {
    pub port_num: c_int,
    pub ch_mask: c_uint,
    pub master_ch_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_common {
    pub dev: *mut device,
    pub max_bias: c_int,
    pub micb_mv: [u32; WCD_MAX_MICBIAS],
    pub micb_vout: [u32; WCD_MAX_MICBIAS],
}

extern "C" {
    pub fn wcd_get_micb_vout_ctl_val(dev: *mut device, micb_mv: u32) -> c_int;
}
extern "C" {
    pub fn wcd_dt_parse_micbias_info(common: *mut wcd_common) -> c_int;
}
extern "C" {
    pub fn wcd_update_status(slave: *mut sdw_slave, status: sdw_slave_status) -> c_int;
}
extern "C" {
    pub fn wcd_bus_config(slave: *mut sdw_slave, params: *mut sdw_bus_params) -> c_int;
}
