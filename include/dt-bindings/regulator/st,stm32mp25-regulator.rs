//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/regulator/st,stm32mp25-regulator.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2024, STMicroelectronics - All Rights Reserved
//
// SCMI voltage domains identifiers
// SOC Internal regulators
pub const VOLTD_SCMI_VDDIO1: c_int = 0;
pub const VOLTD_SCMI_VDDIO2: c_int = 1;
pub const VOLTD_SCMI_VDDIO3: c_int = 2;
pub const VOLTD_SCMI_VDDIO4: c_int = 3;
pub const VOLTD_SCMI_VDDIO: c_int = 4;
pub const VOLTD_SCMI_UCPD: c_int = 5;
pub const VOLTD_SCMI_USB33: c_int = 6;
pub const VOLTD_SCMI_ADC: c_int = 7;
pub const VOLTD_SCMI_GPU: c_int = 8;
pub const VOLTD_SCMI_VREFBUF: c_int = 9;
// STPMIC2 regulators
pub const VOLTD_SCMI_STPMIC2_BUCK1: c_int = 10;
pub const VOLTD_SCMI_STPMIC2_BUCK2: c_int = 11;
pub const VOLTD_SCMI_STPMIC2_BUCK3: c_int = 12;
pub const VOLTD_SCMI_STPMIC2_BUCK4: c_int = 13;
pub const VOLTD_SCMI_STPMIC2_BUCK5: c_int = 14;
pub const VOLTD_SCMI_STPMIC2_BUCK6: c_int = 15;
pub const VOLTD_SCMI_STPMIC2_BUCK7: c_int = 16;
pub const VOLTD_SCMI_STPMIC2_LDO1: c_int = 17;
pub const VOLTD_SCMI_STPMIC2_LDO2: c_int = 18;
pub const VOLTD_SCMI_STPMIC2_LDO3: c_int = 19;
pub const VOLTD_SCMI_STPMIC2_LDO4: c_int = 20;
pub const VOLTD_SCMI_STPMIC2_LDO5: c_int = 21;
pub const VOLTD_SCMI_STPMIC2_LDO6: c_int = 22;
pub const VOLTD_SCMI_STPMIC2_LDO7: c_int = 23;
pub const VOLTD_SCMI_STPMIC2_LDO8: c_int = 24;
pub const VOLTD_SCMI_STPMIC2_REFDDR: c_int = 25;
// External regulators
pub const VOLTD_SCMI_REGU0: c_int = 26;
pub const VOLTD_SCMI_REGU1: c_int = 27;
pub const VOLTD_SCMI_REGU2: c_int = 28;
pub const VOLTD_SCMI_REGU3: c_int = 29;
pub const VOLTD_SCMI_REGU4: c_int = 30;
