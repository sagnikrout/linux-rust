//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/clk/clk_15_0_5_offset.h
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
// Copyright (c) 2026 Advanced Micro Devices, Inc. All rights reserved.

// Macro flag: #define _clk_15_0_5_OFFSET_HEADER
// addressBlock: clk_clk5_0_SmuClkDec
// base address: 0x6c800
pub const regCLK5_CLK0_DS_CNTL: c_uint = 0x4604;
pub const regCLK5_CLK0_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK0_BYPASS_CNTL: c_uint = 0x460a;
pub const regCLK5_CLK0_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK1_DS_CNTL: c_uint = 0x460c;
pub const regCLK5_CLK1_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK1_BYPASS_CNTL: c_uint = 0x4612;
pub const regCLK5_CLK1_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK2_DS_CNTL: c_uint = 0x4614;
pub const regCLK5_CLK2_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK2_BYPASS_CNTL: c_uint = 0x461a;
pub const regCLK5_CLK2_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK3_DS_CNTL: c_uint = 0x461c;
pub const regCLK5_CLK3_DS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK3_BYPASS_CNTL: c_uint = 0x4622;
pub const regCLK5_CLK3_BYPASS_CNTL_BASE_IDX: c_int = 0;
pub const regCLK5_CLK_TICK_CNT_CONFIG_REG: c_uint = 0x4629;
pub const regCLK5_CLK_TICK_CNT_CONFIG_REG_BASE_IDX: c_int = 0;
pub const regCLK5_CLK_TICK_CNT_STATUS: c_uint = 0x462a;
pub const regCLK5_CLK_TICK_CNT_STATUS_BASE_IDX: c_int = 0;
pub const regCLK5_CLK0_CURRENT_CNT: c_uint = 0x462b;
pub const regCLK5_CLK0_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK5_CLK1_CURRENT_CNT: c_uint = 0x462c;
pub const regCLK5_CLK1_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK5_CLK2_CURRENT_CNT: c_uint = 0x462d;
pub const regCLK5_CLK2_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK5_CLK3_CURRENT_CNT: c_uint = 0x462e;
pub const regCLK5_CLK3_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK5_CLK4_CURRENT_CNT: c_uint = 0x462f;
pub const regCLK5_CLK4_CURRENT_CNT_BASE_IDX: c_int = 0;
pub const regCLK5_CLK5_CURRENT_CNT: c_uint = 0x4630;
pub const regCLK5_CLK5_CURRENT_CNT_BASE_IDX: c_int = 0;
