//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ti-lmu-register.h
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
// TI LMU (Lighting Management Unit) Device Register Map
//
// Copyright 2017 Texas Instruments
//
// Author: Milo Kim <milo.kim@ti.com>
//

// LM3631
pub const LM3631_REG_DEVCTRL: c_uint = 0x00;

pub const LM3631_REG_BRT_LSB: c_uint = 0x01;
pub const LM3631_REG_BRT_MSB: c_uint = 0x02;
pub const LM3631_REG_BL_CFG: c_uint = 0x06;

pub const LM3631_BL_DUAL_CHANNEL: c_int = 0;

pub const LM3631_EXPONENTIAL_MAP: c_int = 0;
pub const LM3631_REG_BRT_MODE: c_uint = 0x08;

pub const LM3631_REG_SLOPE: c_uint = 0x09;
pub const LM3631_SLOPE_MASK: c_uint = 0xF0;
pub const LM3631_SLOPE_SHIFT: c_int = 4;
pub const LM3631_REG_LDO_CTRL1: c_uint = 0x0A;

pub const LM3631_REG_LDO_CTRL2: c_uint = 0x0B;

pub const LM3631_REG_VOUT_CONT: c_uint = 0x0C;

pub const LM3631_REG_VOUT_BOOST: c_uint = 0x0C;
pub const LM3631_REG_VOUT_POS: c_uint = 0x0D;
pub const LM3631_REG_VOUT_NEG: c_uint = 0x0E;
pub const LM3631_REG_VOUT_OREF: c_uint = 0x0F;
pub const LM3631_VOUT_MASK: c_uint = 0x3F;
pub const LM3631_REG_ENTIME_VCONT: c_uint = 0x0B;
pub const LM3631_ENTIME_CONT_MASK: c_uint = 0x70;
pub const LM3631_REG_ENTIME_VOREF: c_uint = 0x0F;
pub const LM3631_REG_ENTIME_VPOS: c_uint = 0x10;
pub const LM3631_REG_ENTIME_VNEG: c_uint = 0x11;
pub const LM3631_ENTIME_MASK: c_uint = 0xF0;
pub const LM3631_ENTIME_SHIFT: c_int = 4;
pub const LM3631_MAX_REG: c_uint = 0x16;
// LM3632
pub const LM3632_REG_CONFIG1: c_uint = 0x02;

pub const LM3632_REG_CONFIG2: c_uint = 0x03;

pub const LM3632_REG_BRT_LSB: c_uint = 0x04;
pub const LM3632_REG_BRT_MSB: c_uint = 0x05;
pub const LM3632_REG_IO_CTRL: c_uint = 0x09;

pub const LM3632_I2C_MODE: c_int = 0;

pub const LM3632_REG_ENABLE: c_uint = 0x0A;

pub const LM3632_REG_BIAS_CONFIG: c_uint = 0x0C;

pub const LM3632_REG_VOUT_BOOST: c_uint = 0x0D;
pub const LM3632_REG_VOUT_POS: c_uint = 0x0E;
pub const LM3632_REG_VOUT_NEG: c_uint = 0x0F;
pub const LM3632_VOUT_MASK: c_uint = 0x3F;
pub const LM3632_MAX_REG: c_uint = 0x10;
// LM3633
pub const LM3633_REG_HVLED_OUTPUT_CFG: c_uint = 0x10;

pub const LM3633_HVLED1_CFG_SHIFT: c_int = 0;
pub const LM3633_HVLED2_CFG_SHIFT: c_int = 1;
pub const LM3633_HVLED3_CFG_SHIFT: c_int = 2;
pub const LM3633_REG_BANK_SEL: c_uint = 0x11;
pub const LM3633_REG_BL0_RAMP: c_uint = 0x12;
pub const LM3633_REG_BL1_RAMP: c_uint = 0x13;
pub const LM3633_BL_RAMPUP_MASK: c_uint = 0xF0;
pub const LM3633_BL_RAMPUP_SHIFT: c_int = 4;
pub const LM3633_BL_RAMPDN_MASK: c_uint = 0x0F;
pub const LM3633_BL_RAMPDN_SHIFT: c_int = 0;
pub const LM3633_REG_BL_RAMP_CONF: c_uint = 0x1B;
pub const LM3633_BL_RAMP_MASK: c_uint = 0x0F;
pub const LM3633_BL_RAMP_EACH: c_uint = 0x05;
pub const LM3633_REG_PTN0_RAMP: c_uint = 0x1C;
pub const LM3633_REG_PTN1_RAMP: c_uint = 0x1D;
pub const LM3633_PTN_RAMPUP_MASK: c_uint = 0x70;
pub const LM3633_PTN_RAMPUP_SHIFT: c_int = 4;
pub const LM3633_PTN_RAMPDN_MASK: c_uint = 0x07;
pub const LM3633_PTN_RAMPDN_SHIFT: c_int = 0;
pub const LM3633_REG_LED_MAPPING_MODE: c_uint = 0x1F;

pub const LM3633_REG_IMAX_HVLED_A: c_uint = 0x20;
pub const LM3633_REG_IMAX_HVLED_B: c_uint = 0x21;
pub const LM3633_REG_IMAX_LVLED_BASE: c_uint = 0x22;
pub const LM3633_REG_BL_FEEDBACK_ENABLE: c_uint = 0x28;
pub const LM3633_REG_ENABLE: c_uint = 0x2B;
pub const LM3633_LED_BANK_OFFSET: c_int = 2;
pub const LM3633_REG_PATTERN: c_uint = 0x2C;
pub const LM3633_REG_BOOST_CFG: c_uint = 0x2D;

pub const LM3633_OVP_40V: c_uint = 0x6;
pub const LM3633_REG_PWM_CFG: c_uint = 0x2F;

pub const LM3633_REG_BRT_HVLED_A_LSB: c_uint = 0x40;
pub const LM3633_REG_BRT_HVLED_A_MSB: c_uint = 0x41;
pub const LM3633_REG_BRT_HVLED_B_LSB: c_uint = 0x42;
pub const LM3633_REG_BRT_HVLED_B_MSB: c_uint = 0x43;
pub const LM3633_REG_BRT_LVLED_BASE: c_uint = 0x44;
pub const LM3633_REG_PTN_DELAY: c_uint = 0x50;
pub const LM3633_REG_PTN_LOWTIME: c_uint = 0x51;
pub const LM3633_REG_PTN_HIGHTIME: c_uint = 0x52;
pub const LM3633_REG_PTN_LOWBRT: c_uint = 0x53;

pub const LM3633_REG_BL_OPEN_FAULT_STATUS: c_uint = 0xB0;
pub const LM3633_REG_BL_SHORT_FAULT_STATUS: c_uint = 0xB2;
pub const LM3633_REG_MONITOR_ENABLE: c_uint = 0xB4;
pub const LM3633_MAX_REG: c_uint = 0xB4;
// LM3695
pub const LM3695_REG_GP: c_uint = 0x10;

pub const LM3695_BL_DUAL_CHANNEL: c_int = 0;

pub const LM3695_REG_BRT_LSB: c_uint = 0x13;
pub const LM3695_REG_BRT_MSB: c_uint = 0x14;
pub const LM3695_MAX_REG: c_uint = 0x14;
// LM36274
pub const LM36274_REG_REV: c_uint = 0x01;
pub const LM36274_REG_BL_CFG_1: c_uint = 0x02;
pub const LM36274_REG_BL_CFG_2: c_uint = 0x03;
pub const LM36274_REG_BRT_LSB: c_uint = 0x04;
pub const LM36274_REG_BRT_MSB: c_uint = 0x05;
pub const LM36274_REG_BL_EN: c_uint = 0x08;
pub const LM36274_REG_BIAS_CONFIG_1: c_uint = 0x09;

pub const LM36274_REG_BIAS_CONFIG_2: c_uint = 0x0a;
pub const LM36274_REG_BIAS_CONFIG_3: c_uint = 0x0b;
pub const LM36274_REG_VOUT_BOOST: c_uint = 0x0c;
pub const LM36274_REG_VOUT_POS: c_uint = 0x0d;
pub const LM36274_REG_VOUT_NEG: c_uint = 0x0e;
pub const LM36274_VOUT_MASK: c_uint = 0x3F;
pub const LM36274_MAX_REG: c_uint = 0x13;
