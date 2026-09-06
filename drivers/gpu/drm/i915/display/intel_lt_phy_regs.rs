//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_lt_phy_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//
pub const XE3PLPD_MSGBUS_TIMEOUT_FAST_US: c_int = 500;
pub const XE3PLPD_MACCLK_TURNON_LATENCY_MS: c_int = 2;
pub const XE3PLPD_MACCLK_TURNOFF_LATENCY_US: c_int = 10;
pub const XE3PLPD_RATE_CALIB_DONE_LATENCY_MS: c_int = 1;
pub const XE3PLPD_RESET_START_LATENCY_US: c_int = 10;
pub const XE3PLPD_PWRDN_TO_RDY_LATENCY_US: c_int = 10;
pub const XE3PLPD_RESET_END_LATENCY_MS: c_int = 2;
// LT Phy MAC Register

// LT Phy Pipe Spec Registers

// LT Phy Vendor Register
pub const LT_PHY_VDR_0_CONFIG: c_uint = 0xC02;

pub const LT_PHY_VDR_1_CONFIG: c_uint = 0xC03;

pub const LT_PHY_VDR_2_CONFIG: c_uint = 0xCC3;

pub const LT_PHY_RATE_UPDATE: c_uint = 0xCC4;

pub const PLL_REG4_ADDR: c_uint = 0x8510;
pub const PLL_REG3_ADDR: c_uint = 0x850C;
pub const PLL_REG5_ADDR: c_uint = 0x8514;
pub const PLL_REG57_ADDR: c_uint = 0x85E4;
pub const PLL_LF_ADDR: c_uint = 0x860C;
pub const PLL_TDC_ADDR: c_uint = 0x8610;
pub const PLL_SSC_ADDR: c_uint = 0x8614;
pub const PLL_BIAS2_ADDR: c_uint = 0x8618;
pub const PLL_BIAS_TRIM_ADDR: c_uint = 0x8648;
pub const PLL_DCO_MED_ADDR: c_uint = 0x8640;
pub const PLL_DCO_FINE_ADDR: c_uint = 0x864C;
pub const PLL_SSC_INJ_ADDR: c_uint = 0x8624;
pub const PLL_SURV_BONUS_ADDR: c_uint = 0x8644;
pub const PLL_TYPE_OFFSET: c_uint = 0x200;

