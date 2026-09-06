//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/samsung/s2mps14.h
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
// S2MPS14 registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2mps14_reg {
    S2MPS14_REG_ID,
    S2MPS14_REG_INT1,
    S2MPS14_REG_INT2,
    S2MPS14_REG_INT3,
    S2MPS14_REG_INT1M,
    S2MPS14_REG_INT2M,
    S2MPS14_REG_INT3M,
    S2MPS14_REG_ST1,
    S2MPS14_REG_ST2,
    S2MPS14_REG_PWRONSRC,
    S2MPS14_REG_OFFSRC,
    S2MPS14_REG_BU_CHG,
    S2MPS14_REG_RTCCTRL,
    S2MPS14_REG_CTRL1,
    S2MPS14_REG_CTRL2,
    S2MPS14_REG_RSVD1,
    S2MPS14_REG_RSVD2,
    S2MPS14_REG_RSVD3,
    S2MPS14_REG_RSVD4,
    S2MPS14_REG_RSVD5,
    S2MPS14_REG_RSVD6,
    S2MPS14_REG_CTRL3,
    S2MPS14_REG_RSVD7,
    S2MPS14_REG_RSVD8,
    S2MPS14_REG_WRSTBI,
    S2MPS14_REG_B1CTRL1,
    S2MPS14_REG_B1CTRL2,
    S2MPS14_REG_B2CTRL1,
    S2MPS14_REG_B2CTRL2,
    S2MPS14_REG_B3CTRL1,
    S2MPS14_REG_B3CTRL2,
    S2MPS14_REG_B4CTRL1,
    S2MPS14_REG_B4CTRL2,
    S2MPS14_REG_B5CTRL1,
    S2MPS14_REG_B5CTRL2,
    S2MPS14_REG_L1CTRL,
    S2MPS14_REG_L2CTRL,
    S2MPS14_REG_L3CTRL,
    S2MPS14_REG_L4CTRL,
    S2MPS14_REG_L5CTRL,
    S2MPS14_REG_L6CTRL,
    S2MPS14_REG_L7CTRL,
    S2MPS14_REG_L8CTRL,
    S2MPS14_REG_L9CTRL,
    S2MPS14_REG_L10CTRL,
    S2MPS14_REG_L11CTRL,
    S2MPS14_REG_L12CTRL,
    S2MPS14_REG_L13CTRL,
    S2MPS14_REG_L14CTRL,
    S2MPS14_REG_L15CTRL,
    S2MPS14_REG_L16CTRL,
    S2MPS14_REG_L17CTRL,
    S2MPS14_REG_L18CTRL,
    S2MPS14_REG_L19CTRL,
    S2MPS14_REG_L20CTRL,
    S2MPS14_REG_L21CTRL,
    S2MPS14_REG_L22CTRL,
    S2MPS14_REG_L23CTRL,
    S2MPS14_REG_L24CTRL,
    S2MPS14_REG_L25CTRL,
    S2MPS14_REG_LDODSCH1,
    S2MPS14_REG_LDODSCH2,
    S2MPS14_REG_LDODSCH3,
}

// S2MPS14 regulator ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2mps14_regulators {
    S2MPS14_LDO1,
    S2MPS14_LDO2,
    S2MPS14_LDO3,
    S2MPS14_LDO4,
    S2MPS14_LDO5,
    S2MPS14_LDO6,
    S2MPS14_LDO7,
    S2MPS14_LDO8,
    S2MPS14_LDO9,
    S2MPS14_LDO10,
    S2MPS14_LDO11,
    S2MPS14_LDO12,
    S2MPS14_LDO13,
    S2MPS14_LDO14,
    S2MPS14_LDO15,
    S2MPS14_LDO16,
    S2MPS14_LDO17,
    S2MPS14_LDO18,
    S2MPS14_LDO19,
    S2MPS14_LDO20,
    S2MPS14_LDO21,
    S2MPS14_LDO22,
    S2MPS14_LDO23,
    S2MPS14_LDO24,
    S2MPS14_LDO25,
    S2MPS14_BUCK1,
    S2MPS14_BUCK2,
    S2MPS14_BUCK3,
    S2MPS14_BUCK4,
    S2MPS14_BUCK5,

    S2MPS14_REGULATOR_MAX,
}

// Regulator constraints for BUCKx
pub const S2MPS14_BUCK1235_START_SEL: c_uint = 0x20;
pub const S2MPS14_BUCK4_START_SEL: c_uint = 0x40;
//
// Default ramp delay in uv/us. Datasheet says that ramp delay can be
// controlled however it does not specify which register is used for that.
// Let's assume that default value will be set.
//
pub const S2MPS14_BUCK_RAMP_DELAY: c_int = 12500;
pub const S2MPS14_LDO_VSEL_MASK: c_uint = 0x3F;
pub const S2MPS14_BUCK_VSEL_MASK: c_uint = 0xFF;

pub const S2MPS14_ENABLE_SHIFT: c_int = 6;
// On/Off controlled by PWREN

// On/Off controlled by LDO10EN or EMMCEN

