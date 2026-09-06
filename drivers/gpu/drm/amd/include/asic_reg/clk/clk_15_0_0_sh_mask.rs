//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/clk/clk_15_0_0_sh_mask.h
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
// Copyright 2026 Advanced Micro Devices, Inc.

// Macro flag: #define _clk_15_0_0_SH_MASK_HEADER
// addressBlock: clk_clk8_0_SmuClkDec
// CLK8_CLK_TICK_CNT_CONFIG_REG
pub const CLK8_CLK_TICK_CNT_CONFIG_REG__TIMER_THRESHOLD__SHIFT: c_uint = 0x0;
pub const CLK8_CLK_TICK_CNT_CONFIG_REG__TIMER_THRESHOLD_MASK: c_uint = 0xFFFFL;
// CLK8_CLK0_BYPASS_CNTL
pub const CLK8_CLK0_BYPASS_CNTL__CLK0_BYPASS_SEL__SHIFT: c_uint = 0x0;
pub const CLK8_CLK0_BYPASS_CNTL__CLK0_BYPASS_SEL_MASK: c_uint = 0x00000007L;
// CLK8_CLK1_BYPASS_CNTL
pub const CLK8_CLK1_BYPASS_CNTL__CLK1_BYPASS_SEL__SHIFT: c_uint = 0x0;
pub const CLK8_CLK1_BYPASS_CNTL__CLK1_BYPASS_SEL_MASK: c_uint = 0x00000007L;
// CLK8_CLK2_BYPASS_CNTL
pub const CLK8_CLK2_BYPASS_CNTL__CLK2_BYPASS_SEL__SHIFT: c_uint = 0x0;
pub const CLK8_CLK2_BYPASS_CNTL__CLK2_BYPASS_SEL_MASK: c_uint = 0x00000007L;
// CLK8_CLK3_BYPASS_CNTL
pub const CLK8_CLK3_BYPASS_CNTL__CLK3_BYPASS_SEL__SHIFT: c_uint = 0x0;
pub const CLK8_CLK3_BYPASS_CNTL__CLK3_BYPASS_SEL_MASK: c_uint = 0x00000007L;
// CLK8_CLK4_BYPASS_CNTL
pub const CLK8_CLK4_BYPASS_CNTL__CLK4_BYPASS_SEL__SHIFT: c_uint = 0x0;
pub const CLK8_CLK4_BYPASS_CNTL__CLK4_BYPASS_SEL_MASK: c_uint = 0x00000007L;
// CLK8_CLK0_DS_CNTL
pub const CLK8_CLK0_DS_CNTL__CLK0_DS_DIV_ID__SHIFT: c_uint = 0x0;
pub const CLK8_CLK0_DS_CNTL__CLK0_DS_DIV_ID_MASK: c_uint = 0x0000000FL;
pub const CLK8_CLK0_DS_CNTL__CLK0_ALLOW_DS__SHIFT: c_uint = 0x4;
pub const CLK8_CLK0_DS_CNTL__CLK0_ALLOW_DS_MASK: c_uint = 0x00000010L;
// CLK8_CLK1_DS_CNTL
pub const CLK8_CLK1_DS_CNTL__CLK1_DS_DIV_ID__SHIFT: c_uint = 0x0;
pub const CLK8_CLK1_DS_CNTL__CLK1_DS_DIV_ID_MASK: c_uint = 0x0000000FL;
pub const CLK8_CLK1_DS_CNTL__CLK1_ALLOW_DS__SHIFT: c_uint = 0x4;
pub const CLK8_CLK1_DS_CNTL__CLK1_ALLOW_DS_MASK: c_uint = 0x00000010L;
// CLK8_CLK2_DS_CNTL
pub const CLK8_CLK2_DS_CNTL__CLK2_DS_DIV_ID__SHIFT: c_uint = 0x0;
pub const CLK8_CLK2_DS_CNTL__CLK2_DS_DIV_ID_MASK: c_uint = 0x0000000FL;
pub const CLK8_CLK2_DS_CNTL__CLK2_ALLOW_DS__SHIFT: c_uint = 0x4;
pub const CLK8_CLK2_DS_CNTL__CLK2_ALLOW_DS_MASK: c_uint = 0x00000010L;
// CLK8_CLK3_DS_CNTL
pub const CLK8_CLK3_DS_CNTL__CLK3_DS_DIV_ID__SHIFT: c_uint = 0x0;
pub const CLK8_CLK3_DS_CNTL__CLK3_DS_DIV_ID_MASK: c_uint = 0x0000000FL;
pub const CLK8_CLK3_DS_CNTL__CLK3_ALLOW_DS__SHIFT: c_uint = 0x4;
pub const CLK8_CLK3_DS_CNTL__CLK3_ALLOW_DS_MASK: c_uint = 0x00000010L;
// CLK8_CLK4_DS_CNTL
pub const CLK8_CLK4_DS_CNTL__CLK4_DS_DIV_ID__SHIFT: c_uint = 0x0;
pub const CLK8_CLK4_DS_CNTL__CLK4_DS_DIV_ID_MASK: c_uint = 0x0000000FL;
pub const CLK8_CLK4_DS_CNTL__CLK4_ALLOW_DS__SHIFT: c_uint = 0x4;
pub const CLK8_CLK4_DS_CNTL__CLK4_ALLOW_DS_MASK: c_uint = 0x00000010L;
