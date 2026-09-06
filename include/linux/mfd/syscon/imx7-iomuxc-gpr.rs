//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/imx7-iomuxc-gpr.h
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
// Copyright (C) 2015 Freescale Semiconductor, Inc.
//
pub const IOMUXC_GPR0: c_uint = 0x00;
pub const IOMUXC_GPR1: c_uint = 0x04;
pub const IOMUXC_GPR2: c_uint = 0x08;
pub const IOMUXC_GPR3: c_uint = 0x0c;
pub const IOMUXC_GPR4: c_uint = 0x10;
pub const IOMUXC_GPR5: c_uint = 0x14;
pub const IOMUXC_GPR6: c_uint = 0x18;
pub const IOMUXC_GPR7: c_uint = 0x1c;
pub const IOMUXC_GPR8: c_uint = 0x20;
pub const IOMUXC_GPR9: c_uint = 0x24;
pub const IOMUXC_GPR10: c_uint = 0x28;
pub const IOMUXC_GPR11: c_uint = 0x2c;
pub const IOMUXC_GPR12: c_uint = 0x30;
pub const IOMUXC_GPR13: c_uint = 0x34;
pub const IOMUXC_GPR14: c_uint = 0x38;
pub const IOMUXC_GPR15: c_uint = 0x3c;
pub const IOMUXC_GPR16: c_uint = 0x40;
pub const IOMUXC_GPR17: c_uint = 0x44;
pub const IOMUXC_GPR18: c_uint = 0x48;
pub const IOMUXC_GPR19: c_uint = 0x4c;
pub const IOMUXC_GPR20: c_uint = 0x50;
pub const IOMUXC_GPR21: c_uint = 0x54;
pub const IOMUXC_GPR22: c_uint = 0x58;
// For imx7d iomux gpr register field define

