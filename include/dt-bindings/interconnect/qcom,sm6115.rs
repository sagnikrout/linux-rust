//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,sm6115.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2023, Linaro Limited
//
// BIMC
pub const MASTER_AMPSS_M0: c_int = 0;
pub const MASTER_SNOC_BIMC_RT: c_int = 1;
pub const MASTER_SNOC_BIMC_NRT: c_int = 2;
pub const SNOC_BIMC_MAS: c_int = 3;
pub const MASTER_GRAPHICS_3D: c_int = 4;
pub const MASTER_TCU_0: c_int = 5;
pub const SLAVE_EBI_CH0: c_int = 6;
pub const BIMC_SNOC_SLV: c_int = 7;
// CNOC
pub const SNOC_CNOC_MAS: c_int = 0;
pub const MASTER_QDSS_DAP: c_int = 1;
pub const SLAVE_AHB2PHY_USB: c_int = 2;
pub const SLAVE_APSS_THROTTLE_CFG: c_int = 3;
pub const SLAVE_BIMC_CFG: c_int = 4;
pub const SLAVE_BOOT_ROM: c_int = 5;
pub const SLAVE_CAMERA_NRT_THROTTLE_CFG: c_int = 6;
pub const SLAVE_CAMERA_RT_THROTTLE_CFG: c_int = 7;
pub const SLAVE_CAMERA_CFG: c_int = 8;
pub const SLAVE_CLK_CTL: c_int = 9;
pub const SLAVE_RBCPR_CX_CFG: c_int = 10;
pub const SLAVE_RBCPR_MX_CFG: c_int = 11;
pub const SLAVE_CRYPTO_0_CFG: c_int = 12;
pub const SLAVE_DCC_CFG: c_int = 13;
pub const SLAVE_DDR_PHY_CFG: c_int = 14;
pub const SLAVE_DDR_SS_CFG: c_int = 15;
pub const SLAVE_DISPLAY_CFG: c_int = 16;
pub const SLAVE_DISPLAY_THROTTLE_CFG: c_int = 17;
pub const SLAVE_GPU_CFG: c_int = 18;
pub const SLAVE_GPU_THROTTLE_CFG: c_int = 19;
pub const SLAVE_HWKM_CORE: c_int = 20;
pub const SLAVE_IMEM_CFG: c_int = 21;
pub const SLAVE_IPA_CFG: c_int = 22;
pub const SLAVE_LPASS: c_int = 23;
pub const SLAVE_MAPSS: c_int = 24;
pub const SLAVE_MDSP_MPU_CFG: c_int = 25;
pub const SLAVE_MESSAGE_RAM: c_int = 26;
pub const SLAVE_CNOC_MSS: c_int = 27;
pub const SLAVE_PDM: c_int = 28;
pub const SLAVE_PIMEM_CFG: c_int = 29;
pub const SLAVE_PKA_CORE: c_int = 30;
pub const SLAVE_PMIC_ARB: c_int = 31;
pub const SLAVE_QDSS_CFG: c_int = 32;
pub const SLAVE_QM_CFG: c_int = 33;
pub const SLAVE_QM_MPU_CFG: c_int = 34;
pub const SLAVE_QPIC: c_int = 35;
pub const SLAVE_QUP_0: c_int = 36;
pub const SLAVE_RPM: c_int = 37;
pub const SLAVE_SDCC_1: c_int = 38;
pub const SLAVE_SDCC_2: c_int = 39;
pub const SLAVE_SECURITY: c_int = 40;
pub const SLAVE_SNOC_CFG: c_int = 41;
pub const SLAVE_TCSR: c_int = 42;
pub const SLAVE_TLMM: c_int = 43;
pub const SLAVE_USB3: c_int = 44;
pub const SLAVE_VENUS_CFG: c_int = 45;
pub const SLAVE_VENUS_THROTTLE_CFG: c_int = 46;
pub const SLAVE_VSENSE_CTRL_CFG: c_int = 47;
pub const SLAVE_SERVICE_CNOC: c_int = 48;
// SNOC
pub const MASTER_CRYPTO_CORE0: c_int = 0;
pub const MASTER_SNOC_CFG: c_int = 1;
pub const MASTER_TIC: c_int = 2;
pub const MASTER_ANOC_SNOC: c_int = 3;
pub const BIMC_SNOC_MAS: c_int = 4;
pub const MASTER_PIMEM: c_int = 5;
pub const MASTER_QDSS_BAM: c_int = 6;
pub const MASTER_QPIC: c_int = 7;
pub const MASTER_QUP_0: c_int = 8;
pub const MASTER_IPA: c_int = 9;
pub const MASTER_QDSS_ETR: c_int = 10;
pub const MASTER_SDCC_1: c_int = 11;
pub const MASTER_SDCC_2: c_int = 12;
pub const MASTER_USB3: c_int = 13;
pub const SLAVE_APPSS: c_int = 14;
pub const SNOC_CNOC_SLV: c_int = 15;
pub const SLAVE_OCIMEM: c_int = 16;
pub const SLAVE_PIMEM: c_int = 17;
pub const SNOC_BIMC_SLV: c_int = 18;
pub const SLAVE_SERVICE_SNOC: c_int = 19;
pub const SLAVE_QDSS_STM: c_int = 20;
pub const SLAVE_TCU: c_int = 21;
pub const SLAVE_ANOC_SNOC: c_int = 22;
// CLK Virtual
pub const MASTER_QUP_CORE_0: c_int = 0;
pub const SLAVE_QUP_CORE_0: c_int = 1;
// MMRT Virtual
pub const MASTER_CAMNOC_HF: c_int = 0;
pub const MASTER_MDP_PORT0: c_int = 1;
pub const SLAVE_SNOC_BIMC_RT: c_int = 2;
// MMNRT Virtual
pub const MASTER_CAMNOC_SF: c_int = 0;
pub const MASTER_VIDEO_P0: c_int = 1;
pub const MASTER_VIDEO_PROC: c_int = 2;
pub const SLAVE_SNOC_BIMC_NRT: c_int = 3;
