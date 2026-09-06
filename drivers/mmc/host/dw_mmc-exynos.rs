//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/dw_mmc-exynos.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Exynos Specific Extensions for Synopsys DW Multimedia Card Interface driver
//
// Copyright (C) 2012-2014 Samsung Electronics Co., Ltd.
//
pub const SDMMC_CLKSEL: c_uint = 0x09C;
pub const SDMMC_CLKSEL64: c_uint = 0x0A8;
// Extended Register's Offset
pub const SDMMC_HS400_DQS_EN: c_uint = 0x180;
pub const SDMMC_HS400_ASYNC_FIFO_CTRL: c_uint = 0x184;
pub const SDMMC_HS400_DLINE_CTRL: c_uint = 0x188;
// CLKSEL register defines

// RCLK_EN register defines

// DLINE_CTRL register defines

// Protector Register
pub const SDMMC_EMMCP_BASE: c_uint = 0x1000;

// SMU control defines

// Maximum number of Ending sector
pub const SDMMC_ENDING_SEC_NR_MAX: c_uint = 0xFFFFFFFF;
// Fixed clock divider
pub const EXYNOS4210_FIXED_CIU_CLK_DIV: c_int = 2;
pub const EXYNOS4412_FIXED_CIU_CLK_DIV: c_int = 4;
pub const HS400_FIXED_CIU_CLK_DIV: c_int = 1;
// Minimal required clock frequency for cclkin, unit: HZ
pub const EXYNOS_CCLKIN_MIN: c_int = 50000000;
