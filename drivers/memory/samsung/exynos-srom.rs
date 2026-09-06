//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/samsung/exynos-srom.h
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
// Copyright (c) 2015 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Exynos SROMC register definitions
//

// one register BW holds 4 x 4-bit packed settings for NCS0 - NCS3
pub const EXYNOS_SROM_BW__DATAWIDTH__SHIFT: c_int = 0;
pub const EXYNOS_SROM_BW__ADDRMODE__SHIFT: c_int = 1;
pub const EXYNOS_SROM_BW__WAITENABLE__SHIFT: c_int = 2;
pub const EXYNOS_SROM_BW__BYTEENABLE__SHIFT: c_int = 3;
pub const EXYNOS_SROM_BW__CS_MASK: c_uint = 0xf;
pub const EXYNOS_SROM_BW__NCS0__SHIFT: c_int = 0;
pub const EXYNOS_SROM_BW__NCS1__SHIFT: c_int = 4;
pub const EXYNOS_SROM_BW__NCS2__SHIFT: c_int = 8;
pub const EXYNOS_SROM_BW__NCS3__SHIFT: c_int = 12;
pub const EXYNOS_SROM_BW__NCS4__SHIFT: c_int = 16;
pub const EXYNOS_SROM_BW__NCS5__SHIFT: c_int = 20;
// applies to same to BCS0 - BCS3
pub const EXYNOS_SROM_BCX__PMC__SHIFT: c_int = 0;
pub const EXYNOS_SROM_BCX__TACP__SHIFT: c_int = 4;
pub const EXYNOS_SROM_BCX__TCAH__SHIFT: c_int = 8;
pub const EXYNOS_SROM_BCX__TCOH__SHIFT: c_int = 12;
pub const EXYNOS_SROM_BCX__TACC__SHIFT: c_int = 16;
pub const EXYNOS_SROM_BCX__TCOS__SHIFT: c_int = 24;
pub const EXYNOS_SROM_BCX__TACS__SHIFT: c_int = 28;
