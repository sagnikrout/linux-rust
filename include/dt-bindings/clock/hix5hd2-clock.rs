//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hix5hd2-clock.h
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
// Copyright (c) 2014 Linaro Ltd.
// Copyright (c) 2014 Hisilicon Limited.
//
// fixed rate
pub const HIX5HD2_FIXED_1200M: c_int = 1;
pub const HIX5HD2_FIXED_400M: c_int = 2;
pub const HIX5HD2_FIXED_48M: c_int = 3;
pub const HIX5HD2_FIXED_24M: c_int = 4;
pub const HIX5HD2_FIXED_600M: c_int = 5;
pub const HIX5HD2_FIXED_300M: c_int = 6;
pub const HIX5HD2_FIXED_75M: c_int = 7;
pub const HIX5HD2_FIXED_200M: c_int = 8;
pub const HIX5HD2_FIXED_100M: c_int = 9;
pub const HIX5HD2_FIXED_40M: c_int = 10;
pub const HIX5HD2_FIXED_150M: c_int = 11;
pub const HIX5HD2_FIXED_1728M: c_int = 12;
pub const HIX5HD2_FIXED_28P8M: c_int = 13;
pub const HIX5HD2_FIXED_432M: c_int = 14;
pub const HIX5HD2_FIXED_345P6M: c_int = 15;
pub const HIX5HD2_FIXED_288M: c_int = 16;
pub const HIX5HD2_FIXED_60M: c_int = 17;
pub const HIX5HD2_FIXED_750M: c_int = 18;
pub const HIX5HD2_FIXED_500M: c_int = 19;
pub const HIX5HD2_FIXED_54M: c_int = 20;
pub const HIX5HD2_FIXED_27M: c_int = 21;
pub const HIX5HD2_FIXED_1500M: c_int = 22;
pub const HIX5HD2_FIXED_375M: c_int = 23;
pub const HIX5HD2_FIXED_187M: c_int = 24;
pub const HIX5HD2_FIXED_250M: c_int = 25;
pub const HIX5HD2_FIXED_125M: c_int = 26;
pub const HIX5HD2_FIXED_2P02M: c_int = 27;
pub const HIX5HD2_FIXED_50M: c_int = 28;
pub const HIX5HD2_FIXED_25M: c_int = 29;
pub const HIX5HD2_FIXED_83M: c_int = 30;
// mux clocks
pub const HIX5HD2_SFC_MUX: c_int = 64;
pub const HIX5HD2_MMC_MUX: c_int = 65;
pub const HIX5HD2_FEPHY_MUX: c_int = 66;
pub const HIX5HD2_SD_MUX: c_int = 67;
// gate clocks
pub const HIX5HD2_SFC_RST: c_int = 128;
pub const HIX5HD2_SFC_CLK: c_int = 129;
pub const HIX5HD2_MMC_CIU_CLK: c_int = 130;
pub const HIX5HD2_MMC_BIU_CLK: c_int = 131;
pub const HIX5HD2_MMC_CIU_RST: c_int = 132;
pub const HIX5HD2_FWD_BUS_CLK: c_int = 133;
pub const HIX5HD2_FWD_SYS_CLK: c_int = 134;
pub const HIX5HD2_MAC0_PHY_CLK: c_int = 135;
pub const HIX5HD2_SD_CIU_CLK: c_int = 136;
pub const HIX5HD2_SD_BIU_CLK: c_int = 137;
pub const HIX5HD2_SD_CIU_RST: c_int = 138;
pub const HIX5HD2_WDG0_CLK: c_int = 139;
pub const HIX5HD2_WDG0_RST: c_int = 140;
pub const HIX5HD2_I2C0_CLK: c_int = 141;
pub const HIX5HD2_I2C0_RST: c_int = 142;
pub const HIX5HD2_I2C1_CLK: c_int = 143;
pub const HIX5HD2_I2C1_RST: c_int = 144;
pub const HIX5HD2_I2C2_CLK: c_int = 145;
pub const HIX5HD2_I2C2_RST: c_int = 146;
pub const HIX5HD2_I2C3_CLK: c_int = 147;
pub const HIX5HD2_I2C3_RST: c_int = 148;
pub const HIX5HD2_I2C4_CLK: c_int = 149;
pub const HIX5HD2_I2C4_RST: c_int = 150;
pub const HIX5HD2_I2C5_CLK: c_int = 151;
pub const HIX5HD2_I2C5_RST: c_int = 152;
// complex
pub const HIX5HD2_MAC0_CLK: c_int = 192;
pub const HIX5HD2_MAC1_CLK: c_int = 193;
pub const HIX5HD2_SATA_CLK: c_int = 194;
pub const HIX5HD2_USB_CLK: c_int = 195;
pub const HIX5HD2_NR_CLKS: c_int = 256;
