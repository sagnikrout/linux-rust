//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/regulator/active-semi,8865-regulator.h
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
//
// Device Tree binding constants for the ACT8865 PMIC regulators
//
// These constants should be used to specify regulator modes in device tree for
// ACT8865 regulators as follows:
// ACT8865_REGULATOR_MODE_FIXED:	It is specific to DCDC regulators and it
// specifies the usage of fixed-frequency
// PWM.
//
// ACT8865_REGULATOR_MODE_NORMAL:	It is specific to LDO regulators and it
// specifies the usage of normal mode.
//
// ACT8865_REGULATOR_MODE_LOWPOWER:	For DCDC and LDO regulators; it specify
// the usage of proprietary power-saving
// mode.
//
pub const ACT8865_REGULATOR_MODE_FIXED: c_int = 1;
pub const ACT8865_REGULATOR_MODE_NORMAL: c_int = 2;
pub const ACT8865_REGULATOR_MODE_LOWPOWER: c_int = 3;
