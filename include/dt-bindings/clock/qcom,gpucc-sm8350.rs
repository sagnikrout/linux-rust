//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,gpucc-sm8350.h
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
// Copyright (c) 2022, The Linux Foundation. All rights reserved.
//
// GPU_CC clocks
pub const GPU_CC_AHB_CLK: c_int = 0;
pub const GPU_CC_CB_CLK: c_int = 1;
pub const GPU_CC_CRC_AHB_CLK: c_int = 2;
pub const GPU_CC_CX_APB_CLK: c_int = 3;
pub const GPU_CC_CX_GMU_CLK: c_int = 4;
pub const GPU_CC_CX_QDSS_AT_CLK: c_int = 5;
pub const GPU_CC_CX_QDSS_TRIG_CLK: c_int = 6;
pub const GPU_CC_CX_QDSS_TSCTR_CLK: c_int = 7;
pub const GPU_CC_CX_SNOC_DVM_CLK: c_int = 8;
pub const GPU_CC_CXO_AON_CLK: c_int = 9;
pub const GPU_CC_CXO_CLK: c_int = 10;
pub const GPU_CC_FREQ_MEASURE_CLK: c_int = 11;
pub const GPU_CC_GMU_CLK_SRC: c_int = 12;
pub const GPU_CC_GX_GMU_CLK: c_int = 13;
pub const GPU_CC_GX_QDSS_TSCTR_CLK: c_int = 14;
pub const GPU_CC_GX_VSENSE_CLK: c_int = 15;
pub const GPU_CC_HLOS1_VOTE_GPU_SMMU_CLK: c_int = 16;
pub const GPU_CC_HUB_AHB_DIV_CLK_SRC: c_int = 17;
pub const GPU_CC_HUB_AON_CLK: c_int = 18;
pub const GPU_CC_HUB_CLK_SRC: c_int = 19;
pub const GPU_CC_HUB_CX_INT_CLK: c_int = 20;
pub const GPU_CC_HUB_CX_INT_DIV_CLK_SRC: c_int = 21;
pub const GPU_CC_MND1X_0_GFX3D_CLK: c_int = 22;
pub const GPU_CC_MND1X_1_GFX3D_CLK: c_int = 23;
pub const GPU_CC_PLL0: c_int = 24;
pub const GPU_CC_PLL1: c_int = 25;
pub const GPU_CC_SLEEP_CLK: c_int = 26;
// GPU_CC resets
pub const GPUCC_GPU_CC_ACD_BCR: c_int = 0;
pub const GPUCC_GPU_CC_CB_BCR: c_int = 1;
pub const GPUCC_GPU_CC_CX_BCR: c_int = 2;
pub const GPUCC_GPU_CC_FAST_HUB_BCR: c_int = 3;
pub const GPUCC_GPU_CC_GFX3D_AON_BCR: c_int = 4;
pub const GPUCC_GPU_CC_GMU_BCR: c_int = 5;
pub const GPUCC_GPU_CC_GX_BCR: c_int = 6;
pub const GPUCC_GPU_CC_XO_BCR: c_int = 7;
// GPU_CC GDSCRs
pub const GPU_CX_GDSC: c_int = 0;
pub const GPU_GX_GDSC: c_int = 1;
