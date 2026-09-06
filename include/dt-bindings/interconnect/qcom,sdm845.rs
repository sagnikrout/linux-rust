//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,sdm845.h
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
// Qualcomm SDM845 interconnect IDs
//
// Copyright (c) 2018, Linaro Ltd.
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//
pub const MASTER_A1NOC_CFG: c_int = 0;
pub const MASTER_TSIF: c_int = 1;
pub const MASTER_SDCC_2: c_int = 2;
pub const MASTER_SDCC_4: c_int = 3;
pub const MASTER_UFS_CARD: c_int = 4;
pub const MASTER_UFS_MEM: c_int = 5;
pub const MASTER_PCIE_0: c_int = 6;
pub const SLAVE_A1NOC_SNOC: c_int = 7;
pub const SLAVE_SERVICE_A1NOC: c_int = 8;
pub const SLAVE_ANOC_PCIE_A1NOC_SNOC: c_int = 9;
pub const MASTER_QUP_1: c_int = 10;
pub const MASTER_A2NOC_CFG: c_int = 0;
pub const MASTER_QDSS_BAM: c_int = 1;
pub const MASTER_CNOC_A2NOC: c_int = 2;
pub const MASTER_CRYPTO: c_int = 3;
pub const MASTER_IPA: c_int = 4;
pub const MASTER_PCIE_1: c_int = 5;
pub const MASTER_QDSS_ETR: c_int = 6;
pub const MASTER_USB3_0: c_int = 7;
pub const MASTER_USB3_1: c_int = 8;
pub const SLAVE_A2NOC_SNOC: c_int = 9;
pub const SLAVE_ANOC_PCIE_SNOC: c_int = 10;
pub const SLAVE_SERVICE_A2NOC: c_int = 11;
pub const MASTER_QUP_2: c_int = 12;
pub const MASTER_SPDM: c_int = 0;
pub const MASTER_TIC: c_int = 1;
pub const MASTER_SNOC_CNOC: c_int = 2;
pub const MASTER_QDSS_DAP: c_int = 3;
pub const SLAVE_A1NOC_CFG: c_int = 4;
pub const SLAVE_A2NOC_CFG: c_int = 5;
pub const SLAVE_AOP: c_int = 6;
pub const SLAVE_AOSS: c_int = 7;
pub const SLAVE_CAMERA_CFG: c_int = 8;
pub const SLAVE_CLK_CTL: c_int = 9;
pub const SLAVE_CDSP_CFG: c_int = 10;
pub const SLAVE_RBCPR_CX_CFG: c_int = 11;
pub const SLAVE_CRYPTO_0_CFG: c_int = 12;
pub const SLAVE_DCC_CFG: c_int = 13;
pub const SLAVE_CNOC_DDRSS: c_int = 14;
pub const SLAVE_DISPLAY_CFG: c_int = 15;
pub const SLAVE_GLM: c_int = 16;
pub const SLAVE_GFX3D_CFG: c_int = 17;
pub const SLAVE_IMEM_CFG: c_int = 18;
pub const SLAVE_IPA_CFG: c_int = 19;
pub const SLAVE_CNOC_MNOC_CFG: c_int = 20;
pub const SLAVE_PCIE_0_CFG: c_int = 21;
pub const SLAVE_PCIE_1_CFG: c_int = 22;
pub const SLAVE_PDM: c_int = 23;
pub const SLAVE_SOUTH_PHY_CFG: c_int = 24;
pub const SLAVE_PIMEM_CFG: c_int = 25;
pub const SLAVE_PRNG: c_int = 26;
pub const SLAVE_QDSS_CFG: c_int = 27;
pub const SLAVE_BLSP_2: c_int = 28;
pub const SLAVE_BLSP_1: c_int = 29;
pub const SLAVE_SDCC_2: c_int = 30;
pub const SLAVE_SDCC_4: c_int = 31;
pub const SLAVE_SNOC_CFG: c_int = 32;
pub const SLAVE_SPDM_WRAPPER: c_int = 33;
pub const SLAVE_SPSS_CFG: c_int = 34;
pub const SLAVE_TCSR: c_int = 35;
pub const SLAVE_TLMM_NORTH: c_int = 36;
pub const SLAVE_TLMM_SOUTH: c_int = 37;
pub const SLAVE_TSIF: c_int = 38;
pub const SLAVE_UFS_CARD_CFG: c_int = 39;
pub const SLAVE_UFS_MEM_CFG: c_int = 40;
pub const SLAVE_USB3_0: c_int = 41;
pub const SLAVE_USB3_1: c_int = 42;
pub const SLAVE_VENUS_CFG: c_int = 43;
pub const SLAVE_VSENSE_CTRL_CFG: c_int = 44;
pub const SLAVE_CNOC_A2NOC: c_int = 45;
pub const SLAVE_SERVICE_CNOC: c_int = 46;
pub const MASTER_CNOC_DC_NOC: c_int = 0;
pub const SLAVE_LLCC_CFG: c_int = 1;
pub const SLAVE_MEM_NOC_CFG: c_int = 2;
pub const MASTER_APPSS_PROC: c_int = 0;
pub const MASTER_GNOC_CFG: c_int = 1;
pub const SLAVE_GNOC_SNOC: c_int = 2;
pub const SLAVE_GNOC_MEM_NOC: c_int = 3;
pub const SLAVE_SERVICE_GNOC: c_int = 4;
pub const MASTER_TCU_0: c_int = 0;
pub const MASTER_MEM_NOC_CFG: c_int = 1;
pub const MASTER_GNOC_MEM_NOC: c_int = 2;
pub const MASTER_MNOC_HF_MEM_NOC: c_int = 3;
pub const MASTER_MNOC_SF_MEM_NOC: c_int = 4;
pub const MASTER_SNOC_GC_MEM_NOC: c_int = 5;
pub const MASTER_SNOC_SF_MEM_NOC: c_int = 6;
pub const MASTER_GFX3D: c_int = 7;
pub const SLAVE_MSS_PROC_MS_MPU_CFG: c_int = 8;
pub const SLAVE_MEM_NOC_GNOC: c_int = 9;
pub const SLAVE_LLCC: c_int = 10;
pub const SLAVE_MEM_NOC_SNOC: c_int = 11;
pub const SLAVE_SERVICE_MEM_NOC: c_int = 12;
pub const MASTER_LLCC: c_int = 13;
pub const SLAVE_EBI1: c_int = 14;
pub const MASTER_CNOC_MNOC_CFG: c_int = 0;
pub const MASTER_CAMNOC_HF0: c_int = 1;
pub const MASTER_CAMNOC_HF1: c_int = 2;
pub const MASTER_CAMNOC_SF: c_int = 3;
pub const MASTER_MDP0: c_int = 4;
pub const MASTER_MDP1: c_int = 5;
pub const MASTER_ROTATOR: c_int = 6;
pub const MASTER_VIDEO_P0: c_int = 7;
pub const MASTER_VIDEO_P1: c_int = 8;
pub const MASTER_VIDEO_PROC: c_int = 9;
pub const SLAVE_MNOC_SF_MEM_NOC: c_int = 10;
pub const SLAVE_MNOC_HF_MEM_NOC: c_int = 11;
pub const SLAVE_SERVICE_MNOC: c_int = 12;
pub const MASTER_CAMNOC_HF0_UNCOMP: c_int = 13;
pub const MASTER_CAMNOC_HF1_UNCOMP: c_int = 14;
pub const MASTER_CAMNOC_SF_UNCOMP: c_int = 15;
pub const SLAVE_CAMNOC_UNCOMP: c_int = 16;
pub const MASTER_SNOC_CFG: c_int = 0;
pub const MASTER_A1NOC_SNOC: c_int = 1;
pub const MASTER_A2NOC_SNOC: c_int = 2;
pub const MASTER_GNOC_SNOC: c_int = 3;
pub const MASTER_MEM_NOC_SNOC: c_int = 4;
pub const MASTER_ANOC_PCIE_SNOC: c_int = 5;
pub const MASTER_PIMEM: c_int = 6;
pub const MASTER_GIC: c_int = 7;
pub const SLAVE_APPSS: c_int = 8;
pub const SLAVE_SNOC_CNOC: c_int = 9;
pub const SLAVE_SNOC_MEM_NOC_GC: c_int = 10;
pub const SLAVE_SNOC_MEM_NOC_SF: c_int = 11;
pub const SLAVE_IMEM: c_int = 12;
pub const SLAVE_PCIE_0: c_int = 13;
pub const SLAVE_PCIE_1: c_int = 14;
pub const SLAVE_PIMEM: c_int = 15;
pub const SLAVE_SERVICE_SNOC: c_int = 16;
pub const SLAVE_QDSS_STM: c_int = 17;
pub const SLAVE_TCU: c_int = 18;
