//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn32/dcn32_smu13_driver_if.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2022-2024 Advanced Micro Devices, Inc. All rights reserved.
pub const SMU13_DRIVER_IF_VERSION: c_uint = 0x18;
// Only Clks that have DPM descriptors are listed here
pub const NUM_WM_RANGES: c_int = 4;
// Watermarks
// Table types
pub const TABLE_PMFW_PPTABLE: c_int = 0;
pub const TABLE_COMBO_PPTABLE: c_int = 1;
pub const TABLE_WATERMARKS: c_int = 2;
pub const TABLE_AVFS_PSM_DEBUG: c_int = 3;
pub const TABLE_PMSTATUSLOG: c_int = 4;
pub const TABLE_SMU_METRICS: c_int = 5;
pub const TABLE_DRIVER_SMU_CONFIG: c_int = 6;
pub const TABLE_ACTIVITY_MONITOR_COEFF: c_int = 7;
pub const TABLE_OVERDRIVE: c_int = 8;
pub const TABLE_I2C_COMMANDS: c_int = 9;
pub const TABLE_DRIVER_INFO: c_int = 10;
pub const TABLE_COUNT: c_int = 11;
