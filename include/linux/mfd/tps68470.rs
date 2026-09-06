//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps68470.h
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
// Copyright (C) 2017 Intel Corporation
// Functions to access TPS68470 power management chip.
// Register addresses
pub const TPS68470_REG_POSTDIV2: c_uint = 0x06;
pub const TPS68470_REG_BOOSTDIV: c_uint = 0x07;
pub const TPS68470_REG_BUCKDIV: c_uint = 0x08;
pub const TPS68470_REG_PLLSWR: c_uint = 0x09;
pub const TPS68470_REG_XTALDIV: c_uint = 0x0A;
pub const TPS68470_REG_PLLDIV: c_uint = 0x0B;
pub const TPS68470_REG_POSTDIV: c_uint = 0x0C;
pub const TPS68470_REG_PLLCTL: c_uint = 0x0D;
pub const TPS68470_REG_PLLCTL2: c_uint = 0x0E;
pub const TPS68470_REG_CLKCFG1: c_uint = 0x0F;
pub const TPS68470_REG_CLKCFG2: c_uint = 0x10;
pub const TPS68470_REG_GPCTL0A: c_uint = 0x14;
pub const TPS68470_REG_GPCTL0B: c_uint = 0x15;
pub const TPS68470_REG_GPCTL1A: c_uint = 0x16;
pub const TPS68470_REG_GPCTL1B: c_uint = 0x17;
pub const TPS68470_REG_GPCTL2A: c_uint = 0x18;
pub const TPS68470_REG_GPCTL2B: c_uint = 0x19;
pub const TPS68470_REG_GPCTL3A: c_uint = 0x1A;
pub const TPS68470_REG_GPCTL3B: c_uint = 0x1B;
pub const TPS68470_REG_GPCTL4A: c_uint = 0x1C;
pub const TPS68470_REG_GPCTL4B: c_uint = 0x1D;
pub const TPS68470_REG_GPCTL5A: c_uint = 0x1E;
pub const TPS68470_REG_GPCTL5B: c_uint = 0x1F;
pub const TPS68470_REG_GPCTL6A: c_uint = 0x20;
pub const TPS68470_REG_GPCTL6B: c_uint = 0x21;
pub const TPS68470_REG_SGPO: c_uint = 0x22;
pub const TPS68470_REG_GPDI: c_uint = 0x26;
pub const TPS68470_REG_GPDO: c_uint = 0x27;
pub const TPS68470_REG_VCMVAL: c_uint = 0x3C;
pub const TPS68470_REG_VAUX1VAL: c_uint = 0x3D;
pub const TPS68470_REG_VAUX2VAL: c_uint = 0x3E;
pub const TPS68470_REG_VIOVAL: c_uint = 0x3F;
pub const TPS68470_REG_VSIOVAL: c_uint = 0x40;
pub const TPS68470_REG_VAVAL: c_uint = 0x41;
pub const TPS68470_REG_VDVAL: c_uint = 0x42;
pub const TPS68470_REG_S_I2C_CTL: c_uint = 0x43;
pub const TPS68470_REG_VCMCTL: c_uint = 0x44;
pub const TPS68470_REG_VAUX1CTL: c_uint = 0x45;
pub const TPS68470_REG_VAUX2CTL: c_uint = 0x46;
pub const TPS68470_REG_VACTL: c_uint = 0x47;
pub const TPS68470_REG_VDCTL: c_uint = 0x48;
pub const TPS68470_REG_RESET: c_uint = 0x50;
pub const TPS68470_REG_REVID: c_uint = 0xFF;

// Register field definitions

pub const TPS68470_CLKCFG2_DRV_STR_2MA: c_uint = 0x05;
pub const TPS68470_PLL_OUTPUT_ENABLE: c_uint = 0x02;

pub const TPS68470_OSC_EXT_CAP_DEFAULT: c_uint = 0x05;
pub const TPS68470_OUTPUT_A_SHIFT: c_uint = 0x00;
pub const TPS68470_OUTPUT_B_SHIFT: c_uint = 0x02;

pub const TPS68470_GPIO_MODE_IN: c_int = 0;
pub const TPS68470_GPIO_MODE_IN_PULLUP: c_int = 1;
pub const TPS68470_GPIO_MODE_OUT_CMOS: c_int = 2;
pub const TPS68470_GPIO_MODE_OUT_ODRAIN: c_int = 3;
