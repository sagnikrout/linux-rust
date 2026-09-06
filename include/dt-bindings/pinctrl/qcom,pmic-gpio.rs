//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/qcom,pmic-gpio.h
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
// This header provides constants for the Qualcomm PMIC GPIO binding.
//
pub const PMIC_GPIO_PULL_UP_30: c_int = 0;
pub const PMIC_GPIO_PULL_UP_1P5: c_int = 1;
pub const PMIC_GPIO_PULL_UP_31P5: c_int = 2;
pub const PMIC_GPIO_PULL_UP_1P5_30: c_int = 3;
pub const PMIC_GPIO_STRENGTH_NO: c_int = 0;
pub const PMIC_GPIO_STRENGTH_HIGH: c_int = 1;
pub const PMIC_GPIO_STRENGTH_MED: c_int = 2;
pub const PMIC_GPIO_STRENGTH_LOW: c_int = 3;
//
// Note: PM8018 GPIO3 and GPIO4 are supporting
// only S3 and L2 options (1.8V)
//
pub const PM8018_GPIO_L6: c_int = 0;
pub const PM8018_GPIO_L5: c_int = 1;
pub const PM8018_GPIO_S3: c_int = 2;
pub const PM8018_GPIO_L14: c_int = 3;
pub const PM8018_GPIO_L2: c_int = 4;
pub const PM8018_GPIO_L4: c_int = 5;
pub const PM8018_GPIO_VDD: c_int = 6;
//
// Note: PM8038 GPIO7 and GPIO8 are supporting
// only L11 and L4 options (1.8V)
//
pub const PM8038_GPIO_VPH: c_int = 0;
pub const PM8038_GPIO_BB: c_int = 1;
pub const PM8038_GPIO_L11: c_int = 2;
pub const PM8038_GPIO_L15: c_int = 3;
pub const PM8038_GPIO_L4: c_int = 4;
pub const PM8038_GPIO_L3: c_int = 5;
pub const PM8038_GPIO_L17: c_int = 6;
pub const PM8058_GPIO_VPH: c_int = 0;
pub const PM8058_GPIO_BB: c_int = 1;
pub const PM8058_GPIO_S3: c_int = 2;
pub const PM8058_GPIO_L3: c_int = 3;
pub const PM8058_GPIO_L7: c_int = 4;
pub const PM8058_GPIO_L6: c_int = 5;
pub const PM8058_GPIO_L5: c_int = 6;
pub const PM8058_GPIO_L2: c_int = 7;
//
// Note: PM8916 GPIO1 and GPIO2 are supporting
// only L2(1.15V) and L5(1.8V) options
//
pub const PM8916_GPIO_VPH: c_int = 0;
pub const PM8916_GPIO_L2: c_int = 2;
pub const PM8916_GPIO_L5: c_int = 3;
pub const PM8917_GPIO_VPH: c_int = 0;
pub const PM8917_GPIO_S4: c_int = 2;
pub const PM8917_GPIO_L15: c_int = 3;
pub const PM8917_GPIO_L4: c_int = 4;
pub const PM8917_GPIO_L3: c_int = 5;
pub const PM8917_GPIO_L17: c_int = 6;
pub const PM8921_GPIO_VPH: c_int = 0;
pub const PM8921_GPIO_BB: c_int = 1;
pub const PM8921_GPIO_S4: c_int = 2;
pub const PM8921_GPIO_L15: c_int = 3;
pub const PM8921_GPIO_L4: c_int = 4;
pub const PM8921_GPIO_L3: c_int = 5;
pub const PM8921_GPIO_L17: c_int = 6;
//
// Note: PM8941 gpios from 15 to 18 are supporting
// only S3 and L6 options (1.8V)
//
pub const PM8941_GPIO_VPH: c_int = 0;
pub const PM8941_GPIO_L1: c_int = 1;
pub const PM8941_GPIO_S3: c_int = 2;
pub const PM8941_GPIO_L6: c_int = 3;
//
// Note: PMA8084 gpios from 15 to 18 are supporting
// only S4 and L6 options (1.8V)
//
pub const PMA8084_GPIO_VPH: c_int = 0;
pub const PMA8084_GPIO_L1: c_int = 1;
pub const PMA8084_GPIO_S4: c_int = 2;
pub const PMA8084_GPIO_L6: c_int = 3;
pub const PM8994_GPIO_VPH: c_int = 0;
pub const PM8994_GPIO_S4: c_int = 2;
pub const PM8994_GPIO_L12: c_int = 3;
// To be used with "function"

