//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/samsung/s2mpu02.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd
// http://www.samsung.com
//
// S2MPU02 registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum S2MPU02_reg {
    S2MPU02_REG_ID,
    S2MPU02_REG_INT1,
    S2MPU02_REG_INT2,
    S2MPU02_REG_INT3,
    S2MPU02_REG_INT1M,
    S2MPU02_REG_INT2M,
    S2MPU02_REG_INT3M,
    S2MPU02_REG_ST1,
    S2MPU02_REG_ST2,
    S2MPU02_REG_PWRONSRC,
    S2MPU02_REG_OFFSRC,
    S2MPU02_REG_BU_CHG,
    S2MPU02_REG_RTCCTRL,
    S2MPU02_REG_PMCTRL1,
    S2MPU02_REG_RSVD1,
    S2MPU02_REG_RSVD2,
    S2MPU02_REG_RSVD3,
    S2MPU02_REG_RSVD4,
    S2MPU02_REG_RSVD5,
    S2MPU02_REG_RSVD6,
    S2MPU02_REG_RSVD7,
    S2MPU02_REG_WRSTEN,
    S2MPU02_REG_RSVD8,
    S2MPU02_REG_RSVD9,
    S2MPU02_REG_RSVD10,
    S2MPU02_REG_B1CTRL1,
    S2MPU02_REG_B1CTRL2,
    S2MPU02_REG_B2CTRL1,
    S2MPU02_REG_B2CTRL2,
    S2MPU02_REG_B3CTRL1,
    S2MPU02_REG_B3CTRL2,
    S2MPU02_REG_B4CTRL1,
    S2MPU02_REG_B4CTRL2,
    S2MPU02_REG_B5CTRL1,
    S2MPU02_REG_B5CTRL2,
    S2MPU02_REG_B5CTRL3,
    S2MPU02_REG_B5CTRL4,
    S2MPU02_REG_B5CTRL5,
    S2MPU02_REG_B6CTRL1,
    S2MPU02_REG_B6CTRL2,
    S2MPU02_REG_B7CTRL1,
    S2MPU02_REG_B7CTRL2,
    S2MPU02_REG_RAMP1,
    S2MPU02_REG_RAMP2,
    S2MPU02_REG_L1CTRL,
    S2MPU02_REG_L2CTRL1,
    S2MPU02_REG_L2CTRL2,
    S2MPU02_REG_L2CTRL3,
    S2MPU02_REG_L2CTRL4,
    S2MPU02_REG_L3CTRL,
    S2MPU02_REG_L4CTRL,
    S2MPU02_REG_L5CTRL,
    S2MPU02_REG_L6CTRL,
    S2MPU02_REG_L7CTRL,
    S2MPU02_REG_L8CTRL,
    S2MPU02_REG_L9CTRL,
    S2MPU02_REG_L10CTRL,
    S2MPU02_REG_L11CTRL,
    S2MPU02_REG_L12CTRL,
    S2MPU02_REG_L13CTRL,
    S2MPU02_REG_L14CTRL,
    S2MPU02_REG_L15CTRL,
    S2MPU02_REG_L16CTRL,
    S2MPU02_REG_L17CTRL,
    S2MPU02_REG_L18CTRL,
    S2MPU02_REG_L19CTRL,
    S2MPU02_REG_L20CTRL,
    S2MPU02_REG_L21CTRL,
    S2MPU02_REG_L22CTRL,
    S2MPU02_REG_L23CTRL,
    S2MPU02_REG_L24CTRL,
    S2MPU02_REG_L25CTRL,
    S2MPU02_REG_L26CTRL,
    S2MPU02_REG_L27CTRL,
    S2MPU02_REG_L28CTRL,
    S2MPU02_REG_LDODSCH1,
    S2MPU02_REG_LDODSCH2,
    S2MPU02_REG_LDODSCH3,
    S2MPU02_REG_LDODSCH4,
    S2MPU02_REG_SELMIF,
    S2MPU02_REG_RSVD11,
    S2MPU02_REG_RSVD12,
    S2MPU02_REG_RSVD13,
    S2MPU02_REG_DVSSEL,
    S2MPU02_REG_DVSPTR,
    S2MPU02_REG_DVSDATA,
}

// S2MPU02 regulator ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum S2MPU02_regulators {
    S2MPU02_LDO1,
    S2MPU02_LDO2,
    S2MPU02_LDO3,
    S2MPU02_LDO4,
    S2MPU02_LDO5,
    S2MPU02_LDO6,
    S2MPU02_LDO7,
    S2MPU02_LDO8,
    S2MPU02_LDO9,
    S2MPU02_LDO10,
    S2MPU02_LDO11,
    S2MPU02_LDO12,
    S2MPU02_LDO13,
    S2MPU02_LDO14,
    S2MPU02_LDO15,
    S2MPU02_LDO16,
    S2MPU02_LDO17,
    S2MPU02_LDO18,
    S2MPU02_LDO19,
    S2MPU02_LDO20,
    S2MPU02_LDO21,
    S2MPU02_LDO22,
    S2MPU02_LDO23,
    S2MPU02_LDO24,
    S2MPU02_LDO25,
    S2MPU02_LDO26,
    S2MPU02_LDO27,
    S2MPU02_LDO28,
    S2MPU02_BUCK1,
    S2MPU02_BUCK2,
    S2MPU02_BUCK3,
    S2MPU02_BUCK4,
    S2MPU02_BUCK5,
    S2MPU02_BUCK6,
    S2MPU02_BUCK7,

    S2MPU02_REGULATOR_MAX,
}

// Regulator constraints for BUCKx
pub const S2MPU02_BUCK1234_MIN_600MV: c_int = 600000;
pub const S2MPU02_BUCK5_MIN_1081_25MV: c_int = 1081250;
pub const S2MPU02_BUCK6_MIN_1700MV: c_int = 1700000;
pub const S2MPU02_BUCK7_MIN_900MV: c_int = 900000;
pub const S2MPU02_BUCK1234_STEP_6_25MV: c_int = 6250;
pub const S2MPU02_BUCK5_STEP_6_25MV: c_int = 6250;
pub const S2MPU02_BUCK6_STEP_2_50MV: c_int = 2500;
pub const S2MPU02_BUCK7_STEP_6_25MV: c_int = 6250;
pub const S2MPU02_BUCK1234_START_SEL: c_uint = 0x00;
pub const S2MPU02_BUCK5_START_SEL: c_uint = 0x4D;
pub const S2MPU02_BUCK6_START_SEL: c_uint = 0x28;
pub const S2MPU02_BUCK7_START_SEL: c_uint = 0x30;
pub const S2MPU02_BUCK_RAMP_DELAY: c_int = 12500;
// Regulator constraints for different types of LDOx
pub const S2MPU02_LDO_MIN_900MV: c_int = 900000;
pub const S2MPU02_LDO_MIN_1050MV: c_int = 1050000;
pub const S2MPU02_LDO_MIN_1600MV: c_int = 1600000;
pub const S2MPU02_LDO_STEP_12_5MV: c_int = 12500;
pub const S2MPU02_LDO_STEP_25MV: c_int = 25000;
pub const S2MPU02_LDO_STEP_50MV: c_int = 50000;
pub const S2MPU02_LDO_GROUP1_START_SEL: c_uint = 0x8;
pub const S2MPU02_LDO_GROUP2_START_SEL: c_uint = 0xA;
pub const S2MPU02_LDO_GROUP3_START_SEL: c_uint = 0x10;
pub const S2MPU02_LDO_VSEL_MASK: c_uint = 0x3F;
pub const S2MPU02_BUCK_VSEL_MASK: c_uint = 0xFF;

pub const S2MPU02_ENABLE_SHIFT: c_int = 6;
// On/Off controlled by PWREN

// RAMP delay for BUCK1234
pub const S2MPU02_BUCK1_RAMP_SHIFT: c_int = 6;
pub const S2MPU02_BUCK2_RAMP_SHIFT: c_int = 4;
pub const S2MPU02_BUCK3_RAMP_SHIFT: c_int = 2;
pub const S2MPU02_BUCK4_RAMP_SHIFT: c_int = 0;
pub const S2MPU02_BUCK1234_RAMP_MASK: c_uint = 0x3;
