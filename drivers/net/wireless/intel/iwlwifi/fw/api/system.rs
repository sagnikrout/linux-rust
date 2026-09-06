//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/system.h
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
// Copyright (C) 2012-2014, 2019-2021 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_system_h__

pub const SOC_FLAGS_LTR_APPLY_DELAY_MASK: c_uint = 0xc;
pub const SOC_FLAGS_LTR_APPLY_DELAY_NONE: c_int = 0;
pub const SOC_FLAGS_LTR_APPLY_DELAY_200: c_int = 1;
pub const SOC_FLAGS_LTR_APPLY_DELAY_2500: c_int = 2;
pub const SOC_FLAGS_LTR_APPLY_DELAY_1820: c_int = 3;
//
// struct iwl_soc_configuration_cmd - Set device stabilization latency
//
// @flags: soc settings flags.  In VER_1, we can only set the DISCRETE
// flag, because the FW treats the whole value as an integer. In
// VER_2, we can set the bits independently.
// @latency: time for SOC to ensure stable power & XTAL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_soc_configuration_cmd {
    pub flags: __le32,
    pub latency: __le32,
    pub /*: *mut } __packed;,
// SOC_CONFIGURATION_CMD_S_VER_1 (see description above)
// SOC_CONFIGURATION_CMD_S_VER_2
//
// struct iwl_system_features_control_cmd - system features control command
// @features: bitmap of features to disable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_system_features_control_cmd {
    pub features: [__le32; 4],
    pub /: *mut *mut } __packed; / SYSTEM_FEATURES_CONTROL_CMD_API_S_VER_1,
