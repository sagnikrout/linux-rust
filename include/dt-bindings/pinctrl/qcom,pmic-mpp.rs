//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/qcom,pmic-mpp.h
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
// This header provides constants for the Qualcomm PMIC's
// Multi-Purpose Pin binding.
//
// power-source
// Digital Input/Output: level [PM8058]
pub const PM8058_MPP_VPH: c_int = 0;
pub const PM8058_MPP_S3: c_int = 1;
pub const PM8058_MPP_L2: c_int = 2;
pub const PM8058_MPP_L3: c_int = 3;
// Digital Input/Output: level [PM8901]
pub const PM8901_MPP_MSMIO: c_int = 0;
pub const PM8901_MPP_DIG: c_int = 1;
pub const PM8901_MPP_L5: c_int = 2;
pub const PM8901_MPP_S4: c_int = 3;
pub const PM8901_MPP_VPH: c_int = 4;
// Digital Input/Output: level [PM8921]
pub const PM8921_MPP_S4: c_int = 1;
pub const PM8921_MPP_L15: c_int = 3;
pub const PM8921_MPP_L17: c_int = 4;
pub const PM8921_MPP_VPH: c_int = 7;
// Digital Input/Output: level [PM8821]
pub const PM8821_MPP_1P8: c_int = 0;
pub const PM8821_MPP_VPH: c_int = 7;
// Digital Input/Output: level [PM8018]
pub const PM8018_MPP_L4: c_int = 0;
pub const PM8018_MPP_L14: c_int = 1;
pub const PM8018_MPP_S3: c_int = 2;
pub const PM8018_MPP_L6: c_int = 3;
pub const PM8018_MPP_L2: c_int = 4;
pub const PM8018_MPP_L5: c_int = 5;
pub const PM8018_MPP_VPH: c_int = 7;
// Digital Input/Output: level [PM8038]
pub const PM8038_MPP_L20: c_int = 0;
pub const PM8038_MPP_L11: c_int = 1;
pub const PM8038_MPP_L5: c_int = 2;
pub const PM8038_MPP_L15: c_int = 3;
pub const PM8038_MPP_L17: c_int = 4;
pub const PM8038_MPP_VPH: c_int = 7;
pub const PM8841_MPP_VPH: c_int = 0;
pub const PM8841_MPP_S3: c_int = 2;
pub const PM8916_MPP_VPH: c_int = 0;
pub const PM8916_MPP_L2: c_int = 2;
pub const PM8916_MPP_L5: c_int = 3;
pub const PM8941_MPP_VPH: c_int = 0;
pub const PM8941_MPP_L1: c_int = 1;
pub const PM8941_MPP_S3: c_int = 2;
pub const PM8941_MPP_L6: c_int = 3;
pub const PMA8084_MPP_VPH: c_int = 0;
pub const PMA8084_MPP_L1: c_int = 1;
pub const PMA8084_MPP_S4: c_int = 2;
pub const PMA8084_MPP_L6: c_int = 3;
pub const PM8994_MPP_VPH: c_int = 0;
// Only supported for MPP_05-MPP_08
pub const PM8994_MPP_L19: c_int = 1;
pub const PM8994_MPP_S4: c_int = 2;
pub const PM8994_MPP_L12: c_int = 3;
//
// Analog Input - Set the source for analog input.
// To be used with "qcom,amux-route" property
//
pub const PMIC_MPP_AMUX_ROUTE_CH5: c_int = 0;
pub const PMIC_MPP_AMUX_ROUTE_CH6: c_int = 1;
pub const PMIC_MPP_AMUX_ROUTE_CH7: c_int = 2;
pub const PMIC_MPP_AMUX_ROUTE_CH8: c_int = 3;
pub const PMIC_MPP_AMUX_ROUTE_ABUS1: c_int = 4;
pub const PMIC_MPP_AMUX_ROUTE_ABUS2: c_int = 5;
pub const PMIC_MPP_AMUX_ROUTE_ABUS3: c_int = 6;
pub const PMIC_MPP_AMUX_ROUTE_ABUS4: c_int = 7;
// Analog Output: level
pub const PMIC_MPP_AOUT_LVL_1V25: c_int = 0;
pub const PMIC_MPP_AOUT_LVL_1V25_2: c_int = 1;
pub const PMIC_MPP_AOUT_LVL_0V625: c_int = 2;
pub const PMIC_MPP_AOUT_LVL_0V3125: c_int = 3;
pub const PMIC_MPP_AOUT_LVL_MPP: c_int = 4;
pub const PMIC_MPP_AOUT_LVL_ABUS1: c_int = 5;
pub const PMIC_MPP_AOUT_LVL_ABUS2: c_int = 6;
pub const PMIC_MPP_AOUT_LVL_ABUS3: c_int = 7;
// To be used with "function"

