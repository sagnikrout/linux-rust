//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,nord-nwgcc.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// NW_GCC clocks
pub const NW_GCC_ACMU_MUX_CLK: c_int = 0;
pub const NW_GCC_CAMERA_AHB_CLK: c_int = 1;
pub const NW_GCC_CAMERA_HF_AXI_CLK: c_int = 2;
pub const NW_GCC_CAMERA_SF_AXI_CLK: c_int = 3;
pub const NW_GCC_CAMERA_TRIG_CLK: c_int = 4;
pub const NW_GCC_CAMERA_XO_CLK: c_int = 5;
pub const NW_GCC_DISP_0_AHB_CLK: c_int = 6;
pub const NW_GCC_DISP_0_HF_AXI_CLK: c_int = 7;
pub const NW_GCC_DISP_0_TRIG_CLK: c_int = 8;
pub const NW_GCC_DISP_1_AHB_CLK: c_int = 9;
pub const NW_GCC_DISP_1_HF_AXI_CLK: c_int = 10;
pub const NW_GCC_DISP_1_TRIG_CLK: c_int = 11;
pub const NW_GCC_DPRX0_AXI_HF_CLK: c_int = 12;
pub const NW_GCC_DPRX0_CFG_AHB_CLK: c_int = 13;
pub const NW_GCC_DPRX1_AXI_HF_CLK: c_int = 14;
pub const NW_GCC_DPRX1_CFG_AHB_CLK: c_int = 15;
pub const NW_GCC_EVA_AHB_CLK: c_int = 16;
pub const NW_GCC_EVA_AXI0_CLK: c_int = 17;
pub const NW_GCC_EVA_AXI0C_CLK: c_int = 18;
pub const NW_GCC_EVA_TRIG_CLK: c_int = 19;
pub const NW_GCC_EVA_XO_CLK: c_int = 20;
pub const NW_GCC_FRQ_MEASURE_REF_CLK: c_int = 21;
pub const NW_GCC_GP1_CLK: c_int = 22;
pub const NW_GCC_GP1_CLK_SRC: c_int = 23;
pub const NW_GCC_GP2_CLK: c_int = 24;
pub const NW_GCC_GP2_CLK_SRC: c_int = 25;
pub const NW_GCC_GPLL0: c_int = 26;
pub const NW_GCC_GPLL0_OUT_EVEN: c_int = 27;
pub const NW_GCC_GPU_2_CFG_AHB_CLK: c_int = 28;
pub const NW_GCC_GPU_2_GPLL0_CLK_SRC: c_int = 29;
pub const NW_GCC_GPU_2_GPLL0_DIV_CLK_SRC: c_int = 30;
pub const NW_GCC_GPU_2_HSCNOC_GFX_CLK: c_int = 31;
pub const NW_GCC_GPU_CFG_AHB_CLK: c_int = 32;
pub const NW_GCC_GPU_GPLL0_CLK_SRC: c_int = 33;
pub const NW_GCC_GPU_GPLL0_DIV_CLK_SRC: c_int = 34;
pub const NW_GCC_GPU_HSCNOC_GFX_CLK: c_int = 35;
pub const NW_GCC_GPU_SMMU_VOTE_CLK: c_int = 36;
pub const NW_GCC_HSCNOC_GPU_2_AXI_CLK: c_int = 37;
pub const NW_GCC_HSCNOC_GPU_AXI_CLK: c_int = 38;
pub const NW_GCC_MMU_1_TCU_VOTE_CLK: c_int = 39;
pub const NW_GCC_VIDEO_AHB_CLK: c_int = 40;
pub const NW_GCC_VIDEO_AXI0_CLK: c_int = 41;
pub const NW_GCC_VIDEO_AXI0C_CLK: c_int = 42;
pub const NW_GCC_VIDEO_AXI1_CLK: c_int = 43;
pub const NW_GCC_VIDEO_XO_CLK: c_int = 44;
// NW_GCC power domains
// NW_GCC resets
pub const NW_GCC_CAMERA_BCR: c_int = 0;
pub const NW_GCC_DISPLAY_0_BCR: c_int = 1;
pub const NW_GCC_DISPLAY_1_BCR: c_int = 2;
pub const NW_GCC_DPRX0_BCR: c_int = 3;
pub const NW_GCC_DPRX1_BCR: c_int = 4;
pub const NW_GCC_EVA_BCR: c_int = 5;
pub const NW_GCC_GPU_2_BCR: c_int = 6;
pub const NW_GCC_GPU_BCR: c_int = 7;
pub const NW_GCC_VIDEO_BCR: c_int = 8;
