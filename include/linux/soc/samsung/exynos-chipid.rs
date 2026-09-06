//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/samsung/exynos-chipid.h
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
// Copyright (c) 2018 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Exynos - CHIPID support
//
pub const EXYNOS_CHIPID_REG_PRO_ID: c_uint = 0x00;
pub const EXYNOS_REV_PART_MASK: c_uint = 0xf;
pub const EXYNOS_REV_PART_SHIFT: c_int = 4;
pub const EXYNOS_MASK: c_uint = 0xfffff000;
pub const EXYNOS_CHIPID_REG_PKG_ID: c_uint = 0x04;
// Bit field definitions for EXYNOS_CHIPID_REG_PKG_ID register
pub const EXYNOS5422_IDS_OFFSET: c_int = 24;
pub const EXYNOS5422_IDS_MASK: c_uint = 0xff;
pub const EXYNOS5422_USESG_OFFSET: c_int = 3;
pub const EXYNOS5422_USESG_MASK: c_uint = 0x01;
pub const EXYNOS5422_SG_OFFSET: c_int = 0;
pub const EXYNOS5422_SG_MASK: c_uint = 0x07;
pub const EXYNOS5422_TABLE_OFFSET: c_int = 8;
pub const EXYNOS5422_TABLE_MASK: c_uint = 0x03;
pub const EXYNOS5422_SG_A_OFFSET: c_int = 17;
pub const EXYNOS5422_SG_A_MASK: c_uint = 0x0f;
pub const EXYNOS5422_SG_B_OFFSET: c_int = 21;
pub const EXYNOS5422_SG_B_MASK: c_uint = 0x03;
pub const EXYNOS5422_SG_BSIGN_OFFSET: c_int = 23;
pub const EXYNOS5422_SG_BSIGN_MASK: c_uint = 0x01;
pub const EXYNOS5422_BIN2_OFFSET: c_int = 12;
pub const EXYNOS5422_BIN2_MASK: c_uint = 0x01;
pub const EXYNOS_CHIPID_REG_LOT_ID: c_uint = 0x14;
pub const EXYNOS_CHIPID_REG_AUX_INFO: c_uint = 0x1c;
// Bit field definitions for EXYNOS_CHIPID_REG_AUX_INFO register
pub const EXYNOS5422_TMCB_OFFSET: c_int = 0;
pub const EXYNOS5422_TMCB_MASK: c_uint = 0x7f;
pub const EXYNOS5422_ARM_UP_OFFSET: c_int = 8;
pub const EXYNOS5422_ARM_UP_MASK: c_uint = 0x03;
pub const EXYNOS5422_ARM_DN_OFFSET: c_int = 10;
pub const EXYNOS5422_ARM_DN_MASK: c_uint = 0x03;
pub const EXYNOS5422_KFC_UP_OFFSET: c_int = 12;
pub const EXYNOS5422_KFC_UP_MASK: c_uint = 0x03;
pub const EXYNOS5422_KFC_DN_OFFSET: c_int = 14;
pub const EXYNOS5422_KFC_DN_MASK: c_uint = 0x03;
