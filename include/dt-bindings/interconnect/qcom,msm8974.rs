//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,msm8974.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// Qualcomm msm8974 interconnect IDs
//
// Copyright (c) 2019 Brian Masney <masneyb@onstation.org>
//
pub const BIMC_MAS_AMPSS_M0: c_int = 0;
pub const BIMC_MAS_AMPSS_M1: c_int = 1;
pub const BIMC_MAS_MSS_PROC: c_int = 2;
pub const BIMC_TO_MNOC: c_int = 3;
pub const BIMC_TO_SNOC: c_int = 4;
pub const BIMC_SLV_EBI_CH0: c_int = 5;
pub const BIMC_SLV_AMPSS_L2: c_int = 6;
pub const CNOC_MAS_RPM_INST: c_int = 0;
pub const CNOC_MAS_RPM_DATA: c_int = 1;
pub const CNOC_MAS_RPM_SYS: c_int = 2;
pub const CNOC_MAS_DEHR: c_int = 3;
pub const CNOC_MAS_QDSS_DAP: c_int = 4;
pub const CNOC_MAS_SPDM: c_int = 5;
pub const CNOC_MAS_TIC: c_int = 6;
pub const CNOC_SLV_CLK_CTL: c_int = 7;
pub const CNOC_SLV_CNOC_MSS: c_int = 8;
pub const CNOC_SLV_SECURITY: c_int = 9;
pub const CNOC_SLV_TCSR: c_int = 10;
pub const CNOC_SLV_TLMM: c_int = 11;
pub const CNOC_SLV_CRYPTO_0_CFG: c_int = 12;
pub const CNOC_SLV_CRYPTO_1_CFG: c_int = 13;
pub const CNOC_SLV_IMEM_CFG: c_int = 14;
pub const CNOC_SLV_MESSAGE_RAM: c_int = 15;
pub const CNOC_SLV_BIMC_CFG: c_int = 16;
pub const CNOC_SLV_BOOT_ROM: c_int = 17;
pub const CNOC_SLV_PMIC_ARB: c_int = 18;
pub const CNOC_SLV_SPDM_WRAPPER: c_int = 19;
pub const CNOC_SLV_DEHR_CFG: c_int = 20;
pub const CNOC_SLV_MPM: c_int = 21;
pub const CNOC_SLV_QDSS_CFG: c_int = 22;
pub const CNOC_SLV_RBCPR_CFG: c_int = 23;
pub const CNOC_SLV_RBCPR_QDSS_APU_CFG: c_int = 24;
pub const CNOC_TO_SNOC: c_int = 25;
pub const CNOC_SLV_CNOC_ONOC_CFG: c_int = 26;
pub const CNOC_SLV_CNOC_MNOC_MMSS_CFG: c_int = 27;
pub const CNOC_SLV_CNOC_MNOC_CFG: c_int = 28;
pub const CNOC_SLV_PNOC_CFG: c_int = 29;
pub const CNOC_SLV_SNOC_MPU_CFG: c_int = 30;
pub const CNOC_SLV_SNOC_CFG: c_int = 31;
pub const CNOC_SLV_EBI1_DLL_CFG: c_int = 32;
pub const CNOC_SLV_PHY_APU_CFG: c_int = 33;
pub const CNOC_SLV_EBI1_PHY_CFG: c_int = 34;
pub const CNOC_SLV_RPM: c_int = 35;
pub const CNOC_SLV_SERVICE_CNOC: c_int = 36;
pub const MNOC_MAS_GRAPHICS_3D: c_int = 0;
pub const MNOC_MAS_JPEG: c_int = 1;
pub const MNOC_MAS_MDP_PORT0: c_int = 2;
pub const MNOC_MAS_VIDEO_P0: c_int = 3;
pub const MNOC_MAS_VIDEO_P1: c_int = 4;
pub const MNOC_MAS_VFE: c_int = 5;
pub const MNOC_TO_CNOC: c_int = 6;
pub const MNOC_TO_BIMC: c_int = 7;
pub const MNOC_SLV_CAMERA_CFG: c_int = 8;
pub const MNOC_SLV_DISPLAY_CFG: c_int = 9;
pub const MNOC_SLV_OCMEM_CFG: c_int = 10;
pub const MNOC_SLV_CPR_CFG: c_int = 11;
pub const MNOC_SLV_CPR_XPU_CFG: c_int = 12;
pub const MNOC_SLV_MISC_CFG: c_int = 13;
pub const MNOC_SLV_MISC_XPU_CFG: c_int = 14;
pub const MNOC_SLV_VENUS_CFG: c_int = 15;
pub const MNOC_SLV_GRAPHICS_3D_CFG: c_int = 16;
pub const MNOC_SLV_MMSS_CLK_CFG: c_int = 17;
pub const MNOC_SLV_MMSS_CLK_XPU_CFG: c_int = 18;
pub const MNOC_SLV_MNOC_MPU_CFG: c_int = 19;
pub const MNOC_SLV_ONOC_MPU_CFG: c_int = 20;
pub const MNOC_SLV_SERVICE_MNOC: c_int = 21;
pub const OCMEM_NOC_TO_OCMEM_VNOC: c_int = 0;
pub const OCMEM_MAS_JPEG_OCMEM: c_int = 1;
pub const OCMEM_MAS_MDP_OCMEM: c_int = 2;
pub const OCMEM_MAS_VIDEO_P0_OCMEM: c_int = 3;
pub const OCMEM_MAS_VIDEO_P1_OCMEM: c_int = 4;
pub const OCMEM_MAS_VFE_OCMEM: c_int = 5;
pub const OCMEM_MAS_CNOC_ONOC_CFG: c_int = 6;
pub const OCMEM_SLV_SERVICE_ONOC: c_int = 7;
pub const OCMEM_VNOC_TO_SNOC: c_int = 8;
pub const OCMEM_VNOC_TO_OCMEM_NOC: c_int = 9;
pub const OCMEM_VNOC_MAS_GFX3D: c_int = 10;
pub const OCMEM_SLV_OCMEM: c_int = 11;
pub const PNOC_MAS_PNOC_CFG: c_int = 0;
pub const PNOC_MAS_SDCC_1: c_int = 1;
pub const PNOC_MAS_SDCC_3: c_int = 2;
pub const PNOC_MAS_SDCC_4: c_int = 3;
pub const PNOC_MAS_SDCC_2: c_int = 4;
pub const PNOC_MAS_TSIF: c_int = 5;
pub const PNOC_MAS_BAM_DMA: c_int = 6;
pub const PNOC_MAS_BLSP_2: c_int = 7;
pub const PNOC_MAS_USB_HSIC: c_int = 8;
pub const PNOC_MAS_BLSP_1: c_int = 9;
pub const PNOC_MAS_USB_HS: c_int = 10;
pub const PNOC_TO_SNOC: c_int = 11;
pub const PNOC_SLV_SDCC_1: c_int = 12;
pub const PNOC_SLV_SDCC_3: c_int = 13;
pub const PNOC_SLV_SDCC_2: c_int = 14;
pub const PNOC_SLV_SDCC_4: c_int = 15;
pub const PNOC_SLV_TSIF: c_int = 16;
pub const PNOC_SLV_BAM_DMA: c_int = 17;
pub const PNOC_SLV_BLSP_2: c_int = 18;
pub const PNOC_SLV_USB_HSIC: c_int = 19;
pub const PNOC_SLV_BLSP_1: c_int = 20;
pub const PNOC_SLV_USB_HS: c_int = 21;
pub const PNOC_SLV_PDM: c_int = 22;
pub const PNOC_SLV_PERIPH_APU_CFG: c_int = 23;
pub const PNOC_SLV_PNOC_MPU_CFG: c_int = 24;
pub const PNOC_SLV_PRNG: c_int = 25;
pub const PNOC_SLV_SERVICE_PNOC: c_int = 26;
pub const SNOC_MAS_LPASS_AHB: c_int = 0;
pub const SNOC_MAS_QDSS_BAM: c_int = 1;
pub const SNOC_MAS_SNOC_CFG: c_int = 2;
pub const SNOC_TO_BIMC: c_int = 3;
pub const SNOC_TO_CNOC: c_int = 4;
pub const SNOC_TO_PNOC: c_int = 5;
pub const SNOC_TO_OCMEM_VNOC: c_int = 6;
pub const SNOC_MAS_CRYPTO_CORE0: c_int = 7;
pub const SNOC_MAS_CRYPTO_CORE1: c_int = 8;
pub const SNOC_MAS_LPASS_PROC: c_int = 9;
pub const SNOC_MAS_MSS: c_int = 10;
pub const SNOC_MAS_MSS_NAV: c_int = 11;
pub const SNOC_MAS_OCMEM_DMA: c_int = 12;
pub const SNOC_MAS_WCSS: c_int = 13;
pub const SNOC_MAS_QDSS_ETR: c_int = 14;
pub const SNOC_MAS_USB3: c_int = 15;
pub const SNOC_SLV_AMPSS: c_int = 16;
pub const SNOC_SLV_LPASS: c_int = 17;
pub const SNOC_SLV_USB3: c_int = 18;
pub const SNOC_SLV_WCSS: c_int = 19;
pub const SNOC_SLV_OCIMEM: c_int = 20;
pub const SNOC_SLV_SNOC_OCMEM: c_int = 21;
pub const SNOC_SLV_SERVICE_SNOC: c_int = 22;
pub const SNOC_SLV_QDSS_STM: c_int = 23;
