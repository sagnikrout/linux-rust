//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/interconnect/qcom/msm8996.h
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
// Qualcomm MSM8996 interconnect IDs
//
// Copyright (c) 2021 Yassine Oudjana <y.oudjana@protonmail.com>
//
pub const MSM8996_MASTER_PCIE_0: c_int = 1;
pub const MSM8996_MASTER_PCIE_1: c_int = 2;
pub const MSM8996_MASTER_PCIE_2: c_int = 3;
pub const MSM8996_MASTER_CNOC_A1NOC: c_int = 4;
pub const MSM8996_MASTER_CRYPTO_CORE0: c_int = 5;
pub const MSM8996_MASTER_PNOC_A1NOC: c_int = 6;
pub const MSM8996_MASTER_USB3: c_int = 7;
pub const MSM8996_MASTER_IPA: c_int = 8;
pub const MSM8996_MASTER_UFS: c_int = 9;
pub const MSM8996_MASTER_AMPSS_M0: c_int = 10;
pub const MSM8996_MASTER_GRAPHICS_3D: c_int = 11;
pub const MSM8996_MASTER_MNOC_BIMC: c_int = 12;
pub const MSM8996_MASTER_SNOC_BIMC: c_int = 13;
pub const MSM8996_MASTER_SNOC_CNOC: c_int = 14;
pub const MSM8996_MASTER_QDSS_DAP: c_int = 15;
pub const MSM8996_MASTER_CNOC_MNOC_MMSS_CFG: c_int = 16;
pub const MSM8996_MASTER_CNOC_MNOC_CFG: c_int = 17;
pub const MSM8996_MASTER_CPP: c_int = 18;
pub const MSM8996_MASTER_JPEG: c_int = 19;
pub const MSM8996_MASTER_MDP_PORT0: c_int = 20;
pub const MSM8996_MASTER_MDP_PORT1: c_int = 21;
pub const MSM8996_MASTER_ROTATOR: c_int = 22;
pub const MSM8996_MASTER_VIDEO_P0: c_int = 23;
pub const MSM8996_MASTER_VFE: c_int = 24;
pub const MSM8996_MASTER_SNOC_VMEM: c_int = 25;
pub const MSM8996_MASTER_VIDEO_P0_OCMEM: c_int = 26;
pub const MSM8996_MASTER_SNOC_PNOC: c_int = 27;
pub const MSM8996_MASTER_SDCC_1: c_int = 28;
pub const MSM8996_MASTER_SDCC_2: c_int = 29;
pub const MSM8996_MASTER_SDCC_4: c_int = 30;
pub const MSM8996_MASTER_USB_HS: c_int = 31;
pub const MSM8996_MASTER_BLSP_1: c_int = 32;
pub const MSM8996_MASTER_BLSP_2: c_int = 33;
pub const MSM8996_MASTER_TSIF: c_int = 34;
pub const MSM8996_MASTER_HMSS: c_int = 35;
pub const MSM8996_MASTER_QDSS_BAM: c_int = 36;
pub const MSM8996_MASTER_SNOC_CFG: c_int = 37;
pub const MSM8996_MASTER_BIMC_SNOC_0: c_int = 38;
pub const MSM8996_MASTER_BIMC_SNOC_1: c_int = 39;
pub const MSM8996_MASTER_A0NOC_SNOC: c_int = 40;
pub const MSM8996_MASTER_A1NOC_SNOC: c_int = 41;
pub const MSM8996_MASTER_A2NOC_SNOC: c_int = 42;
pub const MSM8996_MASTER_QDSS_ETR: c_int = 43;
pub const MSM8996_SLAVE_A0NOC_SNOC: c_int = 44;
pub const MSM8996_SLAVE_A1NOC_SNOC: c_int = 45;
pub const MSM8996_SLAVE_A2NOC_SNOC: c_int = 46;
pub const MSM8996_SLAVE_EBI_CH0: c_int = 47;
pub const MSM8996_SLAVE_HMSS_L3: c_int = 48;
pub const MSM8996_SLAVE_BIMC_SNOC_0: c_int = 49;
pub const MSM8996_SLAVE_BIMC_SNOC_1: c_int = 50;
pub const MSM8996_SLAVE_CNOC_A1NOC: c_int = 51;
pub const MSM8996_SLAVE_CLK_CTL: c_int = 52;
pub const MSM8996_SLAVE_TCSR: c_int = 53;
pub const MSM8996_SLAVE_TLMM: c_int = 54;
pub const MSM8996_SLAVE_CRYPTO_0_CFG: c_int = 55;
pub const MSM8996_SLAVE_MPM: c_int = 56;
pub const MSM8996_SLAVE_PIMEM_CFG: c_int = 57;
pub const MSM8996_SLAVE_IMEM_CFG: c_int = 58;
pub const MSM8996_SLAVE_MESSAGE_RAM: c_int = 59;
pub const MSM8996_SLAVE_BIMC_CFG: c_int = 60;
pub const MSM8996_SLAVE_PMIC_ARB: c_int = 61;
pub const MSM8996_SLAVE_PRNG: c_int = 62;
pub const MSM8996_SLAVE_DCC_CFG: c_int = 63;
pub const MSM8996_SLAVE_RBCPR_MX: c_int = 64;
pub const MSM8996_SLAVE_QDSS_CFG: c_int = 65;
pub const MSM8996_SLAVE_RBCPR_CX: c_int = 66;
pub const MSM8996_SLAVE_QDSS_RBCPR_APU_CFG: c_int = 67;
pub const MSM8996_SLAVE_CNOC_MNOC_CFG: c_int = 68;
pub const MSM8996_SLAVE_SNOC_CFG: c_int = 69;
pub const MSM8996_SLAVE_SNOC_MPU_CFG: c_int = 70;
pub const MSM8996_SLAVE_EBI1_PHY_CFG: c_int = 71;
pub const MSM8996_SLAVE_A0NOC_CFG: c_int = 72;
pub const MSM8996_SLAVE_PCIE_1_CFG: c_int = 73;
pub const MSM8996_SLAVE_PCIE_2_CFG: c_int = 74;
pub const MSM8996_SLAVE_PCIE_0_CFG: c_int = 75;
pub const MSM8996_SLAVE_PCIE20_AHB2PHY: c_int = 76;
pub const MSM8996_SLAVE_A0NOC_MPU_CFG: c_int = 77;
pub const MSM8996_SLAVE_UFS_CFG: c_int = 78;
pub const MSM8996_SLAVE_A1NOC_CFG: c_int = 79;
pub const MSM8996_SLAVE_A1NOC_MPU_CFG: c_int = 80;
pub const MSM8996_SLAVE_A2NOC_CFG: c_int = 81;
pub const MSM8996_SLAVE_A2NOC_MPU_CFG: c_int = 82;
pub const MSM8996_SLAVE_SSC_CFG: c_int = 83;
pub const MSM8996_SLAVE_A0NOC_SMMU_CFG: c_int = 84;
pub const MSM8996_SLAVE_A1NOC_SMMU_CFG: c_int = 85;
pub const MSM8996_SLAVE_A2NOC_SMMU_CFG: c_int = 86;
pub const MSM8996_SLAVE_LPASS_SMMU_CFG: c_int = 87;
pub const MSM8996_SLAVE_CNOC_MNOC_MMSS_CFG: c_int = 88;
pub const MSM8996_SLAVE_MMAGIC_CFG: c_int = 89;
pub const MSM8996_SLAVE_CPR_CFG: c_int = 90;
pub const MSM8996_SLAVE_MISC_CFG: c_int = 91;
pub const MSM8996_SLAVE_VENUS_THROTTLE_CFG: c_int = 92;
pub const MSM8996_SLAVE_VENUS_CFG: c_int = 93;
pub const MSM8996_SLAVE_VMEM_CFG: c_int = 94;
pub const MSM8996_SLAVE_DSA_CFG: c_int = 95;
pub const MSM8996_SLAVE_MMSS_CLK_CFG: c_int = 96;
pub const MSM8996_SLAVE_DSA_MPU_CFG: c_int = 97;
pub const MSM8996_SLAVE_MNOC_MPU_CFG: c_int = 98;
pub const MSM8996_SLAVE_DISPLAY_CFG: c_int = 99;
pub const MSM8996_SLAVE_DISPLAY_THROTTLE_CFG: c_int = 100;
pub const MSM8996_SLAVE_CAMERA_CFG: c_int = 101;
pub const MSM8996_SLAVE_CAMERA_THROTTLE_CFG: c_int = 102;
pub const MSM8996_SLAVE_GRAPHICS_3D_CFG: c_int = 103;
pub const MSM8996_SLAVE_SMMU_MDP_CFG: c_int = 104;
pub const MSM8996_SLAVE_SMMU_ROTATOR_CFG: c_int = 105;
pub const MSM8996_SLAVE_SMMU_VENUS_CFG: c_int = 106;
pub const MSM8996_SLAVE_SMMU_CPP_CFG: c_int = 107;
pub const MSM8996_SLAVE_SMMU_JPEG_CFG: c_int = 108;
pub const MSM8996_SLAVE_SMMU_VFE_CFG: c_int = 109;
pub const MSM8996_SLAVE_MNOC_BIMC: c_int = 110;
pub const MSM8996_SLAVE_VMEM: c_int = 111;
pub const MSM8996_SLAVE_SERVICE_MNOC: c_int = 112;
pub const MSM8996_SLAVE_PNOC_A1NOC: c_int = 113;
pub const MSM8996_SLAVE_USB_HS: c_int = 114;
pub const MSM8996_SLAVE_SDCC_2: c_int = 115;
pub const MSM8996_SLAVE_SDCC_4: c_int = 116;
pub const MSM8996_SLAVE_TSIF: c_int = 117;
pub const MSM8996_SLAVE_BLSP_2: c_int = 118;
pub const MSM8996_SLAVE_SDCC_1: c_int = 119;
pub const MSM8996_SLAVE_BLSP_1: c_int = 120;
pub const MSM8996_SLAVE_PDM: c_int = 121;
pub const MSM8996_SLAVE_AHB2PHY: c_int = 122;
pub const MSM8996_SLAVE_APPSS: c_int = 123;
pub const MSM8996_SLAVE_LPASS: c_int = 124;
pub const MSM8996_SLAVE_USB3: c_int = 125;
pub const MSM8996_SLAVE_SNOC_BIMC: c_int = 126;
pub const MSM8996_SLAVE_SNOC_CNOC: c_int = 127;
pub const MSM8996_SLAVE_OCIMEM: c_int = 128;
pub const MSM8996_SLAVE_PIMEM: c_int = 129;
pub const MSM8996_SLAVE_SNOC_VMEM: c_int = 130;
pub const MSM8996_SLAVE_SNOC_PNOC: c_int = 131;
pub const MSM8996_SLAVE_QDSS_STM: c_int = 132;
pub const MSM8996_SLAVE_PCIE_0: c_int = 133;
pub const MSM8996_SLAVE_PCIE_1: c_int = 134;
pub const MSM8996_SLAVE_PCIE_2: c_int = 135;
pub const MSM8996_SLAVE_SERVICE_SNOC: c_int = 136;
