//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77650.h
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
// Copyright (C) 2018 BayLibre SAS
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//
// Common definitions for MAXIM 77650/77651 charger/power-supply.
//

pub const MAX77650_REG_INT_GLBL: c_uint = 0x00;
pub const MAX77650_REG_INT_CHG: c_uint = 0x01;
pub const MAX77650_REG_STAT_CHG_A: c_uint = 0x02;
pub const MAX77650_REG_STAT_CHG_B: c_uint = 0x03;
pub const MAX77650_REG_ERCFLAG: c_uint = 0x04;
pub const MAX77650_REG_STAT_GLBL: c_uint = 0x05;
pub const MAX77650_REG_INTM_GLBL: c_uint = 0x06;
pub const MAX77650_REG_INTM_CHG: c_uint = 0x07;
pub const MAX77650_REG_CNFG_GLBL: c_uint = 0x10;
pub const MAX77650_REG_CID: c_uint = 0x11;
pub const MAX77650_REG_CNFG_GPIO: c_uint = 0x12;
pub const MAX77650_REG_CNFG_CHG_A: c_uint = 0x18;
pub const MAX77650_REG_CNFG_CHG_B: c_uint = 0x19;
pub const MAX77650_REG_CNFG_CHG_C: c_uint = 0x1a;
pub const MAX77650_REG_CNFG_CHG_D: c_uint = 0x1b;
pub const MAX77650_REG_CNFG_CHG_E: c_uint = 0x1c;
pub const MAX77650_REG_CNFG_CHG_F: c_uint = 0x1d;
pub const MAX77650_REG_CNFG_CHG_G: c_uint = 0x1e;
pub const MAX77650_REG_CNFG_CHG_H: c_uint = 0x1f;
pub const MAX77650_REG_CNFG_CHG_I: c_uint = 0x20;
pub const MAX77650_REG_CNFG_SBB_TOP: c_uint = 0x28;
pub const MAX77650_REG_CNFG_SBB0_A: c_uint = 0x29;
pub const MAX77650_REG_CNFG_SBB0_B: c_uint = 0x2a;
pub const MAX77650_REG_CNFG_SBB1_A: c_uint = 0x2b;
pub const MAX77650_REG_CNFG_SBB1_B: c_uint = 0x2c;
pub const MAX77650_REG_CNFG_SBB2_A: c_uint = 0x2d;
pub const MAX77650_REG_CNFG_SBB2_B: c_uint = 0x2e;
pub const MAX77650_REG_CNFG_LDO_A: c_uint = 0x38;
pub const MAX77650_REG_CNFG_LDO_B: c_uint = 0x39;
pub const MAX77650_REG_CNFG_LED0_A: c_uint = 0x40;
pub const MAX77650_REG_CNFG_LED1_A: c_uint = 0x41;
pub const MAX77650_REG_CNFG_LED2_A: c_uint = 0x42;
pub const MAX77650_REG_CNFG_LED0_B: c_uint = 0x43;
pub const MAX77650_REG_CNFG_LED1_B: c_uint = 0x44;
pub const MAX77650_REG_CNFG_LED2_B: c_uint = 0x45;
pub const MAX77650_REG_CNFG_LED_TOP: c_uint = 0x46;

pub const MAX77650_CID_77650A: c_uint = 0x03;
pub const MAX77650_CID_77650C: c_uint = 0x0a;
pub const MAX77650_CID_77651A: c_uint = 0x06;
pub const MAX77650_CID_77651B: c_uint = 0x08;
