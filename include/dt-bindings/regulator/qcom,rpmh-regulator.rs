//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/regulator/qcom,rpmh-regulator.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//
// These mode constants may be used to specify modes for various RPMh regulator
// device tree properties (e.g. regulator-initial-mode).  Each type of regulator
// supports a subset of the possible modes.
//
// %RPMH_REGULATOR_MODE_RET:	Retention mode in which only an extremely small
// load current is allowed.  This mode is supported
// by LDO and SMPS type regulators.
// %RPMH_REGULATOR_MODE_LPM:	Low power mode in which a small load current is
// allowed.  This mode corresponds to PFM for SMPS
// and BOB type regulators.  This mode is supported
// by LDO, HFSMPS, BOB, and PMIC4 FTSMPS type
// regulators.
// %RPMH_REGULATOR_MODE_AUTO:	Auto mode in which the regulator hardware
// automatically switches between LPM and HPM based
// upon the real-time load current.  This mode is
// supported by HFSMPS, BOB, and PMIC4 FTSMPS type
// regulators.
// %RPMH_REGULATOR_MODE_HPM:	High power mode in which the full rated current
// of the regulator is allowed.  This mode
// corresponds to PWM for SMPS and BOB type
// regulators.  This mode is supported by all types
// of regulators.
//
pub const RPMH_REGULATOR_MODE_RET: c_int = 0;
pub const RPMH_REGULATOR_MODE_LPM: c_int = 1;
pub const RPMH_REGULATOR_MODE_AUTO: c_int = 2;
pub const RPMH_REGULATOR_MODE_HPM: c_int = 3;
