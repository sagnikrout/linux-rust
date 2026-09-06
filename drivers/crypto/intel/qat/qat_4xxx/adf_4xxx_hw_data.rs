//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_4xxx/adf_4xxx_hw_data.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const ADF_4XXX_MAX_ACCELENGINES: c_int = 9;

pub const ADF_4XXX_HICPPAGENTCMDPARERRLOG_MASK: c_uint = 0x1F;
pub const ADF_4XXX_PARITYERRORMASK_ATH_CPH_MASK: c_uint = 0xF000F;
pub const ADF_4XXX_PARITYERRORMASK_CPR_XLT_MASK: c_uint = 0x10001;
pub const ADF_4XXX_PARITYERRORMASK_DCPR_UCS_MASK: c_uint = 0x30007;
pub const ADF_4XXX_PARITYERRORMASK_PKE_MASK: c_uint = 0x3F;
//
// SSMFEATREN bit mask
// BIT(4) - enables parity detection on CPP
// BIT(12) - enables the logging of push/pull data errors
// in pperr register
// BIT(16) - BIT(23) - enable parity detection on SPPs
//

// Firmware Binaries

// Firmware for 402XXX

// RL constants
pub const ADF_4XXX_RL_PCIE_SCALE_FACTOR_DIV: c_int = 100;
pub const ADF_4XXX_RL_PCIE_SCALE_FACTOR_MUL: c_int = 102;
pub const ADF_4XXX_RL_DCPR_CORRECTION: c_int = 1;
pub const ADF_4XXX_RL_SCANS_PER_SEC: c_int = 954;

// Clocks frequency

extern "C" {
    pub fn adf_init_hw_data_4xxx(hw_data: *mut adf_hw_device_data, dev_id: u32);
}
extern "C" {
    pub fn adf_clean_hw_data_4xxx(hw_data: *mut adf_hw_device_data);
}
