//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen6_pm.h
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
// Copyright(c) 2025 Intel Corporation

// Power management
pub const ADF_GEN6_PM_POLL_DELAY_US: c_int = 20;

pub const ADF_GEN6_PM_STATUS: c_uint = 0x50A00C;
pub const ADF_GEN6_PM_INTERRUPT: c_uint = 0x50A028;
// Power management source in ERRSOU2 and ERRMSK2

// cpm_pm_interrupt bitfields

pub const ADF_GEN6_PM_DEFAULT_IDLE_FILTER: c_uint = 0x6;
// cpm_pm_status bitfields

// fusectl0 bitfields

// cpm_pm_fw_init bitfields

// ssm_pm_enable bitfield

// ssm_pm_domain_status bitfield

extern "C" {
    pub fn adf_gen6_init_dev_pm_data(accel_dev: *mut adf_accel_dev);
}

