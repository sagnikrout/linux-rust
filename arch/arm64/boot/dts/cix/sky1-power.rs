//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/cix/sky1-power.h
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
// Copyright 2026 Cix Technology Group Co., Ltd.
//
// The Rich OS need flow the macro
pub const SKY1_PD_AUDIO: c_int = 0;
pub const SKY1_PD_PCIE_CTRL0: c_int = 1;
pub const SKY1_PD_PCIE_DUMMY: c_int = 2;
pub const SKY1_PD_PCIEHUB: c_int = 3;
pub const SKY1_PD_MMHUB: c_int = 4;
pub const SKY1_PD_MMHUB_SMMU: c_int = 5;
pub const SKY1_PD_DPU0: c_int = 6;
pub const SKY1_PD_DPU1: c_int = 7;
pub const SKY1_PD_DPU2: c_int = 8;
pub const SKY1_PD_DPU3: c_int = 9;
pub const SKY1_PD_DPU4: c_int = 10;
pub const SKY1_PD_VPU_TOP: c_int = 11;
pub const SKY1_PD_VPU_CORE0: c_int = 12;
pub const SKY1_PD_VPU_CORE1: c_int = 13;
pub const SKY1_PD_VPU_CORE2: c_int = 14;
pub const SKY1_PD_VPU_CORE3: c_int = 15;
pub const SKY1_PD_NPU_CORE0: c_int = 16;
pub const SKY1_PD_NPU_CORE1: c_int = 17;
pub const SKY1_PD_NPU_CORE2: c_int = 18;
pub const SKY1_PD_NPU_TOP: c_int = 19;
pub const SKY1_PD_ISP0: c_int = 20;
pub const SKY1_PD_GPU: c_int = 21;
pub const SKY1_PERF_GPU_CORE: c_int = 0;
pub const SKY1_PERF_GPU_TOP: c_int = 1;
pub const SKY1_PERF_CPU_L: c_int = 2;
pub const SKY1_PERF_CPU_B0: c_int = 3;
pub const SKY1_PERF_CPU_B1: c_int = 4;
pub const SKY1_PERF_CPU_M0: c_int = 5;
pub const SKY1_PERF_CPU_M1: c_int = 6;
pub const SKY1_PERF_DSU: c_int = 7;
pub const SKY1_PERF_NPU: c_int = 8;
pub const SKY1_PERF_VPU: c_int = 9;
pub const SKY1_PERF_CI700: c_int = 10;
pub const SKY1_PERF_NI700: c_int = 11;
