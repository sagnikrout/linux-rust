//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/vlv_dsi_pll_regs.h
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
// Copyright © 2022 Intel Corporation
//

pub const GLK_TX_ESC_CLK_DIV1_MASK: c_uint = 0x3FF;

pub const GLK_TX_ESC_CLK_DIV2_MASK: c_uint = 0x3FF;
pub const BXT_MAX_VAR_OUTPUT_KHZ: c_int = 39500;

pub const BXT_MIPI1_DIV_SHIFT: c_int = 26;
pub const BXT_MIPI2_DIV_SHIFT: c_int = 10;

// TX control divider to select actual TX clock output from (8x/var)
pub const BXT_MIPI1_TX_ESCLK_SHIFT: c_int = 26;
pub const BXT_MIPI2_TX_ESCLK_SHIFT: c_int = 10;

// RX upper control divider to select actual RX clock output from 8x
pub const BXT_MIPI1_RX_ESCLK_UPPER_SHIFT: c_int = 21;
pub const BXT_MIPI2_RX_ESCLK_UPPER_SHIFT: c_int = 5;

// 8/3X divider to select the actual 8/3X clock output from 8x
pub const BXT_MIPI1_8X_BY3_SHIFT: c_int = 19;
pub const BXT_MIPI2_8X_BY3_SHIFT: c_int = 3;

// RX lower control divider to select actual RX clock output from 8x
pub const BXT_MIPI1_RX_ESCLK_LOWER_SHIFT: c_int = 16;
pub const BXT_MIPI2_RX_ESCLK_LOWER_SHIFT: c_int = 0;

pub const RX_DIVIDER_BIT_1_2: c_uint = 0x3;
pub const RX_DIVIDER_BIT_3_4: c_uint = 0xC;

pub const BXT_DSI_PLL_PVD_RATIO_SHIFT: c_int = 16;

pub const BXT_DSI_FREQ_SEL_SHIFT: c_int = 8;

pub const BXT_DSI_PLL_RATIO_MAX: c_uint = 0x7D;
pub const BXT_DSI_PLL_RATIO_MIN: c_uint = 0x22;
pub const GLK_DSI_PLL_RATIO_MAX: c_uint = 0x6F;
pub const GLK_DSI_PLL_RATIO_MIN: c_uint = 0x22;
pub const BXT_DSI_PLL_RATIO_MASK: c_uint = 0xFF;
pub const BXT_REF_CLOCK_KHZ: c_int = 19200;

