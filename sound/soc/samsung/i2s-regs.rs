//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/samsung/i2s-regs.h
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
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Samsung I2S driver's register header
//
pub const I2SCON: c_uint = 0x0;
pub const I2SMOD: c_uint = 0x4;
pub const I2SFIC: c_uint = 0x8;
pub const I2SPSR: c_uint = 0xc;
pub const I2STXD: c_uint = 0x10;
pub const I2SRXD: c_uint = 0x14;
pub const I2SFICS: c_uint = 0x18;
pub const I2STXDS: c_uint = 0x1c;
pub const I2SAHB: c_uint = 0x20;
pub const I2SSTR0: c_uint = 0x24;
pub const I2SSIZE: c_uint = 0x28;
pub const I2STRNCNT: c_uint = 0x2c;
pub const I2SLVL0ADDR: c_uint = 0x30;
pub const I2SLVL1ADDR: c_uint = 0x34;
pub const I2SLVL2ADDR: c_uint = 0x38;
pub const I2SLVL3ADDR: c_uint = 0x3c;
pub const I2SSTR1: c_uint = 0x40;
pub const I2SVER: c_uint = 0x44;
pub const I2SFIC1: c_uint = 0x48;
pub const I2STDM: c_uint = 0x4c;
pub const I2SFSTA: c_uint = 0x50;

pub const MOD_OPCLK_SHIFT: c_int = 30;

pub const MOD_BLCS_SHIFT: c_int = 26;

pub const MOD_BLCP_SHIFT: c_int = 24;

pub const MOD_LRP_SHIFT: c_int = 7;
pub const MOD_LR_LLOW: c_int = 0;
pub const MOD_LR_RLOW: c_int = 1;
pub const MOD_SDF_SHIFT: c_int = 5;
pub const MOD_SDF_IIS: c_int = 0;
pub const MOD_SDF_MSB: c_int = 1;
pub const MOD_SDF_LSB: c_int = 2;
pub const MOD_SDF_MASK: c_int = 3;
pub const MOD_RCLK_SHIFT: c_int = 3;
pub const MOD_RCLK_256FS: c_int = 0;
pub const MOD_RCLK_512FS: c_int = 1;
pub const MOD_RCLK_384FS: c_int = 2;
pub const MOD_RCLK_768FS: c_int = 3;
pub const MOD_RCLK_MASK: c_int = 3;
pub const MOD_BCLK_SHIFT: c_int = 1;
pub const MOD_BCLK_32FS: c_int = 0;
pub const MOD_BCLK_48FS: c_int = 1;
pub const MOD_BCLK_16FS: c_int = 2;
pub const MOD_BCLK_24FS: c_int = 3;
pub const MOD_BCLK_MASK: c_int = 3;

pub const EXYNOS5420_MOD_LRP_SHIFT: c_int = 15;
pub const EXYNOS5420_MOD_SDF_SHIFT: c_int = 6;
pub const EXYNOS5420_MOD_RCLK_SHIFT: c_int = 4;
pub const EXYNOS5420_MOD_BCLK_SHIFT: c_int = 0;
pub const EXYNOS5420_MOD_BCLK_64FS: c_int = 4;
pub const EXYNOS5420_MOD_BCLK_96FS: c_int = 5;
pub const EXYNOS5420_MOD_BCLK_128FS: c_int = 6;
pub const EXYNOS5420_MOD_BCLK_192FS: c_int = 7;
pub const EXYNOS5420_MOD_BCLK_256FS: c_int = 8;
pub const EXYNOS5420_MOD_BCLK_MASK: c_uint = 0xf;
pub const EXYNOS7_MOD_RCLK_64FS: c_int = 4;
pub const EXYNOS7_MOD_RCLK_128FS: c_int = 5;
pub const EXYNOS7_MOD_RCLK_96FS: c_int = 6;
pub const EXYNOS7_MOD_RCLK_192FS: c_int = 7;

