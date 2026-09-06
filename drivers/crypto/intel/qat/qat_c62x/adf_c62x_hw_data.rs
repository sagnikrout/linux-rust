//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_c62x/adf_c62x_hw_data.h
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

// PCIe configuration space
pub const ADF_C62X_SRAM_BAR: c_int = 0;
pub const ADF_C62X_PMISC_BAR: c_int = 1;
pub const ADF_C62X_ETR_BAR: c_int = 2;
pub const ADF_C62X_MAX_ACCELERATORS: c_int = 5;
pub const ADF_C62X_MAX_ACCELENGINES: c_int = 10;
pub const ADF_C62X_ACCELERATORS_REG_OFFSET: c_int = 16;
pub const ADF_C62X_ACCELERATORS_MASK: c_uint = 0x1F;
pub const ADF_C62X_ACCELENGINES_MASK: c_uint = 0x3FF;
pub const ADF_C62X_ETR_MAX_BANKS: c_int = 16;
pub const ADF_C62X_SOFTSTRAP_CSR_OFFSET: c_uint = 0x2EC;
// AE to function mapping
pub const ADF_C62X_AE2FUNC_MAP_GRP_A_NUM_REGS: c_int = 80;
pub const ADF_C62X_AE2FUNC_MAP_GRP_B_NUM_REGS: c_int = 10;
// Clocks frequency

// Firmware Binary

extern "C" {
    pub fn adf_init_hw_data_c62x(hw_data: *mut adf_hw_device_data);
}
extern "C" {
    pub fn adf_clean_hw_data_c62x(hw_data: *mut adf_hw_device_data);
}
