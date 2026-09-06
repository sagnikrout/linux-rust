//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,msm8939.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Qualcomm interconnect IDs
//
// Copyright (c) 2020, Linaro Ltd.
// Author: Jun Nie <jun.nie@linaro.org>
//
pub const BIMC_SNOC_SLV: c_int = 0;
pub const MASTER_QDSS_BAM: c_int = 1;
pub const MASTER_QDSS_ETR: c_int = 2;
pub const MASTER_SNOC_CFG: c_int = 3;
pub const PCNOC_SNOC_SLV: c_int = 4;
pub const SLAVE_APSS: c_int = 5;
pub const SLAVE_CATS_128: c_int = 6;
pub const SLAVE_OCMEM_64: c_int = 7;
pub const SLAVE_IMEM: c_int = 8;
pub const SLAVE_QDSS_STM: c_int = 9;
pub const SLAVE_SRVC_SNOC: c_int = 10;
pub const SNOC_BIMC_0_MAS: c_int = 11;
pub const SNOC_BIMC_1_MAS: c_int = 12;
pub const SNOC_BIMC_2_MAS: c_int = 13;
pub const SNOC_INT_0: c_int = 14;
pub const SNOC_INT_1: c_int = 15;
pub const SNOC_INT_BIMC: c_int = 16;
pub const SNOC_PCNOC_MAS: c_int = 17;
pub const SNOC_QDSS_INT: c_int = 18;
pub const MASTER_VIDEO_P0: c_int = 0;
pub const MASTER_JPEG: c_int = 1;
pub const MASTER_VFE: c_int = 2;
pub const MASTER_MDP_PORT0: c_int = 3;
pub const MASTER_MDP_PORT1: c_int = 4;
pub const MASTER_CPP: c_int = 5;
pub const SNOC_MM_INT_0: c_int = 6;
pub const SNOC_MM_INT_1: c_int = 7;
pub const SNOC_MM_INT_2: c_int = 8;
pub const BIMC_SNOC_MAS: c_int = 0;
pub const MASTER_AMPSS_M0: c_int = 1;
pub const MASTER_GRAPHICS_3D: c_int = 2;
pub const MASTER_TCU0: c_int = 3;
pub const SLAVE_AMPSS_L2: c_int = 4;
pub const SLAVE_EBI_CH0: c_int = 5;
pub const SNOC_BIMC_0_SLV: c_int = 6;
pub const SNOC_BIMC_1_SLV: c_int = 7;
pub const SNOC_BIMC_2_SLV: c_int = 8;
pub const MASTER_BLSP_1: c_int = 0;
pub const MASTER_DEHR: c_int = 1;
pub const MASTER_LPASS: c_int = 2;
pub const MASTER_CRYPTO_CORE0: c_int = 3;
pub const MASTER_SDCC_1: c_int = 4;
pub const MASTER_SDCC_2: c_int = 5;
pub const MASTER_SPDM: c_int = 6;
pub const MASTER_USB_HS1: c_int = 7;
pub const MASTER_USB_HS2: c_int = 8;
pub const PCNOC_INT_0: c_int = 9;
pub const PCNOC_INT_1: c_int = 10;
pub const PCNOC_MAS_0: c_int = 11;
pub const PCNOC_MAS_1: c_int = 12;
pub const PCNOC_SLV_0: c_int = 13;
pub const PCNOC_SLV_1: c_int = 14;
pub const PCNOC_SLV_2: c_int = 15;
pub const PCNOC_SLV_3: c_int = 16;
pub const PCNOC_SLV_4: c_int = 17;
pub const PCNOC_SLV_8: c_int = 18;
pub const PCNOC_SLV_9: c_int = 19;
pub const PCNOC_SNOC_MAS: c_int = 20;
pub const SLAVE_BIMC_CFG: c_int = 21;
pub const SLAVE_BLSP_1: c_int = 22;
pub const SLAVE_BOOT_ROM: c_int = 23;
pub const SLAVE_CAMERA_CFG: c_int = 24;
pub const SLAVE_CLK_CTL: c_int = 25;
pub const SLAVE_CRYPTO_0_CFG: c_int = 26;
pub const SLAVE_DEHR_CFG: c_int = 27;
pub const SLAVE_DISPLAY_CFG: c_int = 28;
pub const SLAVE_GRAPHICS_3D_CFG: c_int = 29;
pub const SLAVE_IMEM_CFG: c_int = 30;
pub const SLAVE_LPASS: c_int = 31;
pub const SLAVE_MPM: c_int = 32;
pub const SLAVE_MSG_RAM: c_int = 33;
pub const SLAVE_MSS: c_int = 34;
pub const SLAVE_PDM: c_int = 35;
pub const SLAVE_PMIC_ARB: c_int = 36;
pub const SLAVE_PCNOC_CFG: c_int = 37;
pub const SLAVE_PRNG: c_int = 38;
pub const SLAVE_QDSS_CFG: c_int = 39;
pub const SLAVE_RBCPR_CFG: c_int = 40;
pub const SLAVE_SDCC_1: c_int = 41;
pub const SLAVE_SDCC_2: c_int = 42;
pub const SLAVE_SECURITY: c_int = 43;
pub const SLAVE_SNOC_CFG: c_int = 44;
pub const SLAVE_SPDM: c_int = 45;
pub const SLAVE_TCSR: c_int = 46;
pub const SLAVE_TLMM: c_int = 47;
pub const SLAVE_USB_HS1: c_int = 48;
pub const SLAVE_USB_HS2: c_int = 49;
pub const SLAVE_VENUS_CFG: c_int = 50;
pub const SNOC_PCNOC_SLV: c_int = 51;
