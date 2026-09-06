//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen2_hw_data.h
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
// Copyright(c) 2020 Intel Corporation

pub const ADF_GEN2_RX_RINGS_OFFSET: c_int = 8;
pub const ADF_GEN2_TX_RINGS_MASK: c_uint = 0xFF;
// AE to function map

pub const AE2FUNCTION_MAP_REG_SIZE: c_int = 4;

// Admin Interface Offsets

pub const ADF_MAILBOX_BASE_OFFSET: c_uint = 0x20970;
// Arbiter configuration
pub const ADF_ARB_OFFSET: c_uint = 0x30000;
pub const ADF_ARB_WRK_2_SER_MAP_OFFSET: c_uint = 0x180;

// Power gating

// Default ring mapping

// WDT timers
//
// Timeout is in cycles. Clock speed may vary across products but this
// value should be a few milli-seconds.
//
pub const ADF_SSM_WDT_DEFAULT_VALUE: c_uint = 0x200000;
pub const ADF_SSM_WDT_PKE_DEFAULT_VALUE: c_uint = 0x2000000;
pub const ADF_SSMWDT_OFFSET: c_uint = 0x54;
pub const ADF_SSMWDTPKE_OFFSET: c_uint = 0x58;

// Error detection and correction

// Number of heartbeat counter pairs

// Interrupts

pub const ADF_GEN2_SMIA1_MASK: c_uint = 0x1;
extern "C" {
    pub fn adf_gen2_get_num_accels(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen2_get_num_aes(self: *mut adf_hw_device_data) -> u32;
}
extern "C" {
    pub fn adf_gen2_enable_error_correction(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen2_get_admin_info(admin_csrs_info: *mut admin_info);
}
extern "C" {
    pub fn adf_gen2_get_arb_info(arb_info: *mut arb_info);
}
extern "C" {
    pub fn adf_gen2_enable_ints(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen2_get_accel_cap(accel_dev: *mut adf_accel_dev) -> u32;
}
extern "C" {
    pub fn adf_gen2_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_gen2_init_dc_ops(dc_ops: *mut adf_dc_ops);
}
