//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lp873x.h
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
//
// Functions to access LP873X power management chip.
//
// Copyright (C) 2016 Texas Instruments Incorporated - https://www.ti.com
//

// LP873x chip id list
pub const LP873X: c_uint = 0x00;
// All register addresses

pub const LP873X_REG_LDO_STAT: c_uint = 0x1F;
pub const LP873X_REG_TOP_MASK_1: c_uint = 0x20;
pub const LP873X_REG_TOP_MASK_2: c_uint = 0x21;
pub const LP873X_REG_BUCK_MASK: c_uint = 0x22;
pub const LP873X_REG_LDO_MASK: c_uint = 0x23;
pub const LP873X_REG_SEL_I_LOAD: c_uint = 0x24;
pub const LP873X_REG_I_LOAD_2: c_uint = 0x25;
pub const LP873X_REG_I_LOAD_1: c_uint = 0x26;

// Register field definitions
pub const LP873X_DEV_REV_DEV_ID: c_uint = 0xC0;
pub const LP873X_DEV_REV_ALL_LAYER: c_uint = 0x30;
pub const LP873X_DEV_REV_METAL_LAYER: c_uint = 0x0F;
pub const LP873X_OTP_REV_OTP_ID: c_uint = 0xFF;

pub const LP873X_BUCK0_CTRL_2_BUCK0_ILIM: c_uint = 0x38;
pub const LP873X_BUCK0_CTRL_2_BUCK0_SLEW_RATE: c_uint = 0x07;

pub const LP873X_BUCK1_CTRL_2_BUCK1_ILIM: c_uint = 0x38;
pub const LP873X_BUCK1_CTRL_2_BUCK1_SLEW_RATE: c_uint = 0x07;
pub const LP873X_BUCK0_VOUT_BUCK0_VSET: c_uint = 0xFF;
pub const LP873X_BUCK1_VOUT_BUCK1_VSET: c_uint = 0xFF;

pub const LP873X_LDO0_VOUT_LDO0_VSET: c_uint = 0x1F;
pub const LP873X_LDO1_VOUT_LDO1_VSET: c_uint = 0x1F;
pub const LP873X_BUCK0_DELAY_BUCK0_SD_DELAY: c_uint = 0xF0;
pub const LP873X_BUCK0_DELAY_BUCK0_SU_DELAY: c_uint = 0x0F;
pub const LP873X_BUCK1_DELAY_BUCK1_SD_DELAY: c_uint = 0xF0;
pub const LP873X_BUCK1_DELAY_BUCK1_SU_DELAY: c_uint = 0x0F;
pub const LP873X_LDO0_DELAY_LDO0_SD_DELAY: c_uint = 0xF0;
pub const LP873X_LDO0_DELAY_LDO0_SU_DELAY: c_uint = 0x0F;
pub const LP873X_LDO1_DELAY_LDO1_SD_DELAY: c_uint = 0xF0;
pub const LP873X_LDO1_DELAY_LDO1_SU_DELAY: c_uint = 0x0F;
pub const LP873X_GPO_DELAY_GPO_SD_DELAY: c_uint = 0xF0;
pub const LP873X_GPO_DELAY_GPO_SU_DELAY: c_uint = 0x0F;
pub const LP873X_GPO2_DELAY_GPO2_SD_DELAY: c_uint = 0xF0;
pub const LP873X_GPO2_DELAY_GPO2_SU_DELAY: c_uint = 0x0F;

pub const LP873X_EXT_CLK_FREQ: c_uint = 0x1F;

pub const LP873X_I_LOAD_1_BUCK_LOAD_CURRENT: c_uint = 0xFF;

// Number of step-down converters available
pub const LP873X_NUM_BUCK: c_int = 2;
// Number of LDO voltage regulators available
pub const LP873X_NUM_LDO: c_int = 2;
// Number of total regulators available

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp873x_regulator_id {
// BUCK's
    LP873X_BUCK_0,
    LP873X_BUCK_1,
// LDOs
    LP873X_LDO_0,
    LP873X_LDO_1,
}

//
// struct lp873x - state holder for the lp873x driver
// @dev: struct device pointer for MFD device
// @rev: revision of the lp873x
// @lock: lock guarding the data structure
// @regmap: register map of the lp873x PMIC
//
// Device data may be used to access the LP873X chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp873x {
    pub dev: *mut device,
    pub rev: u8,
    pub regmap: *mut regmap,
}
