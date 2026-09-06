//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,qcs615-gpucc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
//
// GPU_CC clocks
pub const CRC_DIV_PLL0: c_int = 0;
pub const CRC_DIV_PLL1: c_int = 1;
pub const GPU_CC_PLL0: c_int = 2;
pub const GPU_CC_PLL1: c_int = 3;
pub const GPU_CC_CRC_AHB_CLK: c_int = 4;
pub const GPU_CC_CX_GFX3D_CLK: c_int = 5;
pub const GPU_CC_CX_GFX3D_SLV_CLK: c_int = 6;
pub const GPU_CC_CX_GMU_CLK: c_int = 7;
pub const GPU_CC_CX_SNOC_DVM_CLK: c_int = 8;
pub const GPU_CC_CXO_AON_CLK: c_int = 9;
pub const GPU_CC_CXO_CLK: c_int = 10;
pub const GPU_CC_GMU_CLK_SRC: c_int = 11;
pub const GPU_CC_GX_GFX3D_CLK: c_int = 12;
pub const GPU_CC_GX_GFX3D_CLK_SRC: c_int = 13;
pub const GPU_CC_GX_GMU_CLK: c_int = 14;
pub const GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK: c_int = 15;
pub const GPU_CC_SLEEP_CLK: c_int = 16;
// GPU_CC power domains
pub const CX_GDSC: c_int = 0;
pub const GX_GDSC: c_int = 1;
// GPU_CC resets
pub const GPU_CC_CX_BCR: c_int = 0;
pub const GPU_CC_GFX3D_AON_BCR: c_int = 1;
pub const GPU_CC_GMU_BCR: c_int = 2;
pub const GPU_CC_GX_BCR: c_int = 3;
pub const GPU_CC_XO_BCR: c_int = 4;
