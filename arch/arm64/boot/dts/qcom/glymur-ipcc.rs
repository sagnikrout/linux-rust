//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/qcom/glymur-ipcc.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Glymur physical client IDs
pub const IPCC_MPROC_AOP: c_int = 0;
pub const IPCC_MPROC_TZ: c_int = 1;
pub const IPCC_MPROC_MPSS: c_int = 2;
pub const IPCC_MPROC_LPASS: c_int = 3;
pub const IPCC_MPROC_SLPI: c_int = 4;
pub const IPCC_MPROC_SDC: c_int = 5;
pub const IPCC_MPROC_CDSP: c_int = 6;
pub const IPCC_MPROC_NPU: c_int = 7;
pub const IPCC_MPROC_APSS: c_int = 8;
pub const IPCC_MPROC_GPU: c_int = 9;
pub const IPCC_MPROC_ICP: c_int = 11;
pub const IPCC_MPROC_VPU: c_int = 12;
pub const IPCC_MPROC_PCIE0: c_int = 13;
pub const IPCC_MPROC_PCIE1: c_int = 14;
pub const IPCC_MPROC_PCIE2: c_int = 15;
pub const IPCC_MPROC_SPSS: c_int = 16;
pub const IPCC_MPROC_PCIE3: c_int = 19;
pub const IPCC_MPROC_PCIE4: c_int = 20;
pub const IPCC_MPROC_PCIE5: c_int = 21;
pub const IPCC_MPROC_PCIE6: c_int = 22;
pub const IPCC_MPROC_TME: c_int = 23;
pub const IPCC_MPROC_WPSS: c_int = 24;
pub const IPCC_MPROC_PCIE7: c_int = 44;
pub const IPCC_MPROC_SOCCP: c_int = 46;
pub const IPCC_COMPUTE_L0_LPASS: c_int = 0;
pub const IPCC_COMPUTE_L0_CDSP: c_int = 1;
pub const IPCC_COMPUTE_L0_APSS: c_int = 2;
pub const IPCC_COMPUTE_L0_GPU: c_int = 3;
pub const IPCC_COMPUTE_L0_CVP: c_int = 6;
pub const IPCC_COMPUTE_L0_ICP: c_int = 7;
pub const IPCC_COMPUTE_L0_VPU: c_int = 8;
pub const IPCC_COMPUTE_L0_DPU: c_int = 9;
pub const IPCC_COMPUTE_L0_SOCCP: c_int = 11;
pub const IPCC_COMPUTE_L1_LPASS: c_int = 0;
pub const IPCC_COMPUTE_L1_CDSP: c_int = 1;
pub const IPCC_COMPUTE_L1_APSS: c_int = 2;
pub const IPCC_COMPUTE_L1_GPU: c_int = 3;
pub const IPCC_COMPUTE_L1_CVP: c_int = 6;
pub const IPCC_COMPUTE_L1_ICP: c_int = 7;
pub const IPCC_COMPUTE_L1_VPU: c_int = 8;
pub const IPCC_COMPUTE_L1_DPU: c_int = 9;
pub const IPCC_COMPUTE_L1_SOCCP: c_int = 11;
pub const IPCC_PERIPH_LPASS: c_int = 0;
pub const IPCC_PERIPH_APSS: c_int = 1;
pub const IPCC_PERIPH_PCIE0: c_int = 2;
pub const IPCC_PERIPH_PCIE1: c_int = 3;
pub const IPCC_PERIPH_PCIE2: c_int = 6;
pub const IPCC_PERIPH_PCIE3: c_int = 7;
pub const IPCC_PERIPH_PCIE4: c_int = 8;
pub const IPCC_PERIPH_PCIE5: c_int = 9;
pub const IPCC_PERIPH_PCIE6: c_int = 10;
pub const IPCC_PERIPH_PCIE7: c_int = 11;
pub const IPCC_PERIPH_SOCCP: c_int = 13;
pub const IPCC_PERIPH_WPSS: c_int = 16;
