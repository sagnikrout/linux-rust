//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,msm8996.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Qualcomm MSM8996 interconnect IDs
//
// Copyright (c) 2021 Yassine Oudjana <y.oudjana@protonmail.com>
//
// A0NOC
pub const MASTER_PCIE_0: c_int = 0;
pub const MASTER_PCIE_1: c_int = 1;
pub const MASTER_PCIE_2: c_int = 2;
// A1NOC
pub const MASTER_CNOC_A1NOC: c_int = 0;
pub const MASTER_CRYPTO_CORE0: c_int = 1;
pub const MASTER_PNOC_A1NOC: c_int = 2;
// A2NOC
pub const MASTER_USB3: c_int = 0;
pub const MASTER_IPA: c_int = 1;
pub const MASTER_UFS: c_int = 2;
// BIMC
pub const MASTER_AMPSS_M0: c_int = 0;
pub const MASTER_GRAPHICS_3D: c_int = 1;
pub const MASTER_MNOC_BIMC: c_int = 2;
pub const MASTER_SNOC_BIMC: c_int = 3;
pub const SLAVE_EBI_CH0: c_int = 4;
pub const SLAVE_HMSS_L3: c_int = 5;
pub const SLAVE_BIMC_SNOC_0: c_int = 6;
pub const SLAVE_BIMC_SNOC_1: c_int = 7;
// CNOC
pub const MASTER_SNOC_CNOC: c_int = 0;
pub const MASTER_QDSS_DAP: c_int = 1;
pub const SLAVE_CNOC_A1NOC: c_int = 2;
pub const SLAVE_CLK_CTL: c_int = 3;
pub const SLAVE_TCSR: c_int = 4;
pub const SLAVE_TLMM: c_int = 5;
pub const SLAVE_CRYPTO_0_CFG: c_int = 6;
pub const SLAVE_MPM: c_int = 7;
pub const SLAVE_PIMEM_CFG: c_int = 8;
pub const SLAVE_IMEM_CFG: c_int = 9;
pub const SLAVE_MESSAGE_RAM: c_int = 10;
pub const SLAVE_BIMC_CFG: c_int = 11;
pub const SLAVE_PMIC_ARB: c_int = 12;
pub const SLAVE_PRNG: c_int = 13;
pub const SLAVE_DCC_CFG: c_int = 14;
pub const SLAVE_RBCPR_MX: c_int = 15;
pub const SLAVE_QDSS_CFG: c_int = 16;
pub const SLAVE_RBCPR_CX: c_int = 17;
pub const SLAVE_QDSS_RBCPR_APU: c_int = 18;
pub const SLAVE_CNOC_MNOC_CFG: c_int = 19;
pub const SLAVE_SNOC_CFG: c_int = 20;
pub const SLAVE_SNOC_MPU_CFG: c_int = 21;
pub const SLAVE_EBI1_PHY_CFG: c_int = 22;
pub const SLAVE_A0NOC_CFG: c_int = 23;
pub const SLAVE_PCIE_1_CFG: c_int = 24;
pub const SLAVE_PCIE_2_CFG: c_int = 25;
pub const SLAVE_PCIE_0_CFG: c_int = 26;
pub const SLAVE_PCIE20_AHB2PHY: c_int = 27;
pub const SLAVE_A0NOC_MPU_CFG: c_int = 28;
pub const SLAVE_UFS_CFG: c_int = 29;
pub const SLAVE_A1NOC_CFG: c_int = 30;
pub const SLAVE_A1NOC_MPU_CFG: c_int = 31;
pub const SLAVE_A2NOC_CFG: c_int = 32;
pub const SLAVE_A2NOC_MPU_CFG: c_int = 33;
pub const SLAVE_SSC_CFG: c_int = 34;
pub const SLAVE_A0NOC_SMMU_CFG: c_int = 35;
pub const SLAVE_A1NOC_SMMU_CFG: c_int = 36;
pub const SLAVE_A2NOC_SMMU_CFG: c_int = 37;
pub const SLAVE_LPASS_SMMU_CFG: c_int = 38;
pub const SLAVE_CNOC_MNOC_MMSS_CFG: c_int = 39;
// MNOC
pub const MASTER_CNOC_MNOC_CFG: c_int = 0;
pub const MASTER_CPP: c_int = 1;
pub const MASTER_JPEG: c_int = 2;
pub const MASTER_MDP_PORT0: c_int = 3;
pub const MASTER_MDP_PORT1: c_int = 4;
pub const MASTER_ROTATOR: c_int = 5;
pub const MASTER_VIDEO_P0: c_int = 6;
pub const MASTER_VFE: c_int = 7;
pub const MASTER_SNOC_VMEM: c_int = 8;
pub const MASTER_VIDEO_P0_OCMEM: c_int = 9;
pub const MASTER_CNOC_MNOC_MMSS_CFG: c_int = 10;
pub const SLAVE_MNOC_BIMC: c_int = 11;
pub const SLAVE_VMEM: c_int = 12;
pub const SLAVE_SERVICE_MNOC: c_int = 13;
pub const SLAVE_MMAGIC_CFG: c_int = 14;
pub const SLAVE_CPR_CFG: c_int = 15;
pub const SLAVE_MISC_CFG: c_int = 16;
pub const SLAVE_VENUS_THROTTLE_CFG: c_int = 17;
pub const SLAVE_VENUS_CFG: c_int = 18;
pub const SLAVE_VMEM_CFG: c_int = 19;
pub const SLAVE_DSA_CFG: c_int = 20;
pub const SLAVE_MMSS_CLK_CFG: c_int = 21;
pub const SLAVE_DSA_MPU_CFG: c_int = 22;
pub const SLAVE_MNOC_MPU_CFG: c_int = 23;
pub const SLAVE_DISPLAY_CFG: c_int = 24;
pub const SLAVE_DISPLAY_THROTTLE_CFG: c_int = 25;
pub const SLAVE_CAMERA_CFG: c_int = 26;
pub const SLAVE_CAMERA_THROTTLE_CFG: c_int = 27;
pub const SLAVE_GRAPHICS_3D_CFG: c_int = 28;
pub const SLAVE_SMMU_MDP_CFG: c_int = 29;
pub const SLAVE_SMMU_ROT_CFG: c_int = 30;
pub const SLAVE_SMMU_VENUS_CFG: c_int = 31;
pub const SLAVE_SMMU_CPP_CFG: c_int = 32;
pub const SLAVE_SMMU_JPEG_CFG: c_int = 33;
pub const SLAVE_SMMU_VFE_CFG: c_int = 34;
// PNOC
pub const MASTER_SNOC_PNOC: c_int = 0;
pub const MASTER_SDCC_1: c_int = 1;
pub const MASTER_SDCC_2: c_int = 2;
pub const MASTER_SDCC_4: c_int = 3;
pub const MASTER_USB_HS: c_int = 4;
pub const MASTER_BLSP_1: c_int = 5;
pub const MASTER_BLSP_2: c_int = 6;
pub const MASTER_TSIF: c_int = 7;
pub const SLAVE_PNOC_A1NOC: c_int = 8;
pub const SLAVE_USB_HS: c_int = 9;
pub const SLAVE_SDCC_2: c_int = 10;
pub const SLAVE_SDCC_4: c_int = 11;
pub const SLAVE_TSIF: c_int = 12;
pub const SLAVE_BLSP_2: c_int = 13;
pub const SLAVE_SDCC_1: c_int = 14;
pub const SLAVE_BLSP_1: c_int = 15;
pub const SLAVE_PDM: c_int = 16;
pub const SLAVE_AHB2PHY: c_int = 17;
// SNOC
pub const MASTER_HMSS: c_int = 0;
pub const MASTER_QDSS_BAM: c_int = 1;
pub const MASTER_SNOC_CFG: c_int = 2;
pub const MASTER_BIMC_SNOC_0: c_int = 3;
pub const MASTER_BIMC_SNOC_1: c_int = 4;
pub const MASTER_A0NOC_SNOC: c_int = 5;
pub const MASTER_A1NOC_SNOC: c_int = 6;
pub const MASTER_A2NOC_SNOC: c_int = 7;
pub const MASTER_QDSS_ETR: c_int = 8;
pub const SLAVE_A0NOC_SNOC: c_int = 9;
pub const SLAVE_A1NOC_SNOC: c_int = 10;
pub const SLAVE_A2NOC_SNOC: c_int = 11;
pub const SLAVE_HMSS: c_int = 12;
pub const SLAVE_LPASS: c_int = 13;
pub const SLAVE_USB3: c_int = 14;
pub const SLAVE_SNOC_BIMC: c_int = 15;
pub const SLAVE_SNOC_CNOC: c_int = 16;
pub const SLAVE_IMEM: c_int = 17;
pub const SLAVE_PIMEM: c_int = 18;
pub const SLAVE_SNOC_VMEM: c_int = 19;
pub const SLAVE_SNOC_PNOC: c_int = 20;
pub const SLAVE_QDSS_STM: c_int = 21;
pub const SLAVE_PCIE_0: c_int = 22;
pub const SLAVE_PCIE_1: c_int = 23;
pub const SLAVE_PCIE_2: c_int = 24;
pub const SLAVE_SERVICE_SNOC: c_int = 25;
