//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_420xx/adf_420xx_hw_data.h
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
// Copyright(c) 2023 Intel Corporation

pub const ADF_420XX_MAX_ACCELENGINES: c_int = 17;
pub const ADF_420XX_ACCELENGINES_MASK: c_uint = 0x1FFFF;
pub const ADF_420XX_ADMIN_AE_MASK: c_uint = 0x10000;

//
// SSMFEATREN bit mask
// BIT(4) - enables parity detection on CPP
// BIT(12) - enables the logging of push/pull data errors
// in pperr register
// BIT(16) - BIT(27) - enable parity detection on SPPs
//

// Firmware Binaries

// RL constants
pub const ADF_420XX_RL_PCIE_SCALE_FACTOR_DIV: c_int = 100;
pub const ADF_420XX_RL_PCIE_SCALE_FACTOR_MUL: c_int = 102;
pub const ADF_420XX_RL_DCPR_CORRECTION: c_int = 1;
pub const ADF_420XX_RL_SCANS_PER_SEC: c_int = 954;

// Clocks frequency

extern "C" {
    pub fn adf_init_hw_data_420xx(hw_data: *mut adf_hw_device_data, dev_id: u32);
}
extern "C" {
    pub fn adf_clean_hw_data_420xx(hw_data: *mut adf_hw_device_data);
}
