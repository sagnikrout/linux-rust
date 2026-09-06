//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/clk/clk_15_0_0_offset.h
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

// Macro flag: #define _clk_15_0_0_OFFSET_HEADER
// addressBlock: clk_clk8_0_SmuClkDec
// base address: 0x6e000
pub const regCLK8_CLK0_DS_CNTL: c_uint = 0x4c14;
pub const regCLK8_CLK0_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK1_DS_CNTL: c_uint = 0x4c1c;
pub const regCLK8_CLK1_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK2_DS_CNTL: c_uint = 0x4c24;
pub const regCLK8_CLK2_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK3_DS_CNTL: c_uint = 0x4c2c;
pub const regCLK8_CLK3_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK4_DS_CNTL: c_uint = 0x4c34;
pub const regCLK8_CLK4_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK0_BYPASS_CNTL: c_uint = 0x4c1a;
pub const regCLK8_CLK0_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK1_BYPASS_CNTL: c_uint = 0x4c22;
pub const regCLK8_CLK1_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK2_BYPASS_CNTL: c_uint = 0x4c2a;
pub const regCLK8_CLK2_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK3_BYPASS_CNTL: c_uint = 0x4c32;
pub const regCLK8_CLK3_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK4_BYPASS_CNTL: c_uint = 0x4c3a;
pub const regCLK8_CLK4_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK8_CLK_TICK_CNT_CONFIG_REG: c_uint = 0x4c51;
pub const regCLK8_CLK_TICK_CNT_CONFIG_REG_BASE_IDX: c_int = 0;
pub const regCLK8_CLK_TICK_CNT_STATUS: c_uint = 0x4c52;
pub const regCLK8_CLK_TICK_CNT_STATUS_BASE_IDX: c_int = 0;
pub const regCLK8_CLK0_CURRENT_CNT: c_uint = 0x4c53;
pub const regCLK8_CLK0_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK8_CLK1_CURRENT_CNT: c_uint = 0x4c54;
pub const regCLK8_CLK1_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK8_CLK2_CURRENT_CNT: c_uint = 0x4c55;
pub const regCLK8_CLK2_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK8_CLK3_CURRENT_CNT: c_uint = 0x4c56;
pub const regCLK8_CLK3_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK8_CLK4_CURRENT_CNT: c_uint = 0x4c57;
pub const regCLK8_CLK4_CURRENT_CNT_BASE_IDX: c_int = 0;
