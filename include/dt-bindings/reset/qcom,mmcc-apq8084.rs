//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,mmcc-apq8084.h
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
//
pub const MMSS_SPDM_RESET: c_int = 0;
pub const MMSS_SPDM_RM_RESET: c_int = 1;
pub const VENUS0_RESET: c_int = 2;
pub const VPU_RESET: c_int = 3;
pub const MDSS_RESET: c_int = 4;
pub const AVSYNC_RESET: c_int = 5;
pub const CAMSS_PHY0_RESET: c_int = 6;
pub const CAMSS_PHY1_RESET: c_int = 7;
pub const CAMSS_PHY2_RESET: c_int = 8;
pub const CAMSS_CSI0_RESET: c_int = 9;
pub const CAMSS_CSI0PHY_RESET: c_int = 10;
pub const CAMSS_CSI0RDI_RESET: c_int = 11;
pub const CAMSS_CSI0PIX_RESET: c_int = 12;
pub const CAMSS_CSI1_RESET: c_int = 13;
pub const CAMSS_CSI1PHY_RESET: c_int = 14;
pub const CAMSS_CSI1RDI_RESET: c_int = 15;
pub const CAMSS_CSI1PIX_RESET: c_int = 16;
pub const CAMSS_CSI2_RESET: c_int = 17;
pub const CAMSS_CSI2PHY_RESET: c_int = 18;
pub const CAMSS_CSI2RDI_RESET: c_int = 19;
pub const CAMSS_CSI2PIX_RESET: c_int = 20;
pub const CAMSS_CSI3_RESET: c_int = 21;
pub const CAMSS_CSI3PHY_RESET: c_int = 22;
pub const CAMSS_CSI3RDI_RESET: c_int = 23;
pub const CAMSS_CSI3PIX_RESET: c_int = 24;
pub const CAMSS_ISPIF_RESET: c_int = 25;
pub const CAMSS_CCI_RESET: c_int = 26;
pub const CAMSS_MCLK0_RESET: c_int = 27;
pub const CAMSS_MCLK1_RESET: c_int = 28;
pub const CAMSS_MCLK2_RESET: c_int = 29;
pub const CAMSS_MCLK3_RESET: c_int = 30;
pub const CAMSS_GP0_RESET: c_int = 31;
pub const CAMSS_GP1_RESET: c_int = 32;
pub const CAMSS_TOP_RESET: c_int = 33;
pub const CAMSS_AHB_RESET: c_int = 34;
pub const CAMSS_MICRO_RESET: c_int = 35;
pub const CAMSS_JPEG_RESET: c_int = 36;
pub const CAMSS_VFE_RESET: c_int = 37;
pub const CAMSS_CSI_VFE0_RESET: c_int = 38;
pub const CAMSS_CSI_VFE1_RESET: c_int = 39;
pub const OXILI_RESET: c_int = 40;
pub const OXILICX_RESET: c_int = 41;
pub const OCMEMCX_RESET: c_int = 42;
pub const MMSS_RBCRP_RESET: c_int = 43;
pub const MMSSNOCAHB_RESET: c_int = 44;
pub const MMSSNOCAXI_RESET: c_int = 45;
