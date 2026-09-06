//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mediatek,mt6795-resets.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-2-Clause)
//
// Copyright (c) 2022 Collabora Ltd.
// Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
// INFRACFG resets
pub const MT6795_INFRA_RST0_SCPSYS_RST: c_int = 0;
pub const MT6795_INFRA_RST0_PMIC_WRAP_RST: c_int = 1;
pub const MT6795_INFRA_RST1_MIPI_DSI_RST: c_int = 2;
pub const MT6795_INFRA_RST1_MIPI_CSI_RST: c_int = 3;
pub const MT6795_INFRA_RST1_MM_IOMMU_RST: c_int = 4;
// MMSYS resets
pub const MT6795_MMSYS_SW0_RST_B_SMI_COMMON: c_int = 0;
pub const MT6795_MMSYS_SW0_RST_B_SMI_LARB: c_int = 1;
pub const MT6795_MMSYS_SW0_RST_B_CAM_MDP: c_int = 2;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RDMA0: c_int = 3;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RDMA1: c_int = 4;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RSZ0: c_int = 5;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RSZ1: c_int = 6;
pub const MT6795_MMSYS_SW0_RST_B_MDP_RSZ2: c_int = 7;
pub const MT6795_MMSYS_SW0_RST_B_MDP_TDSHP0: c_int = 8;
pub const MT6795_MMSYS_SW0_RST_B_MDP_TDSHP1: c_int = 9;
pub const MT6795_MMSYS_SW0_RST_B_MDP_WDMA: c_int = 10;
pub const MT6795_MMSYS_SW0_RST_B_MDP_WROT0: c_int = 11;
pub const MT6795_MMSYS_SW0_RST_B_MDP_WROT1: c_int = 12;
pub const MT6795_MMSYS_SW0_RST_B_MDP_CROP: c_int = 13;
// PERICFG resets
pub const MT6795_PERI_NFI_SW_RST: c_int = 0;
pub const MT6795_PERI_THERM_SW_RST: c_int = 1;
pub const MT6795_PERI_MSDC1_SW_RST: c_int = 2;
// TOPRGU resets
pub const MT6795_TOPRGU_INFRA_SW_RST: c_int = 0;
pub const MT6795_TOPRGU_MM_SW_RST: c_int = 1;
pub const MT6795_TOPRGU_MFG_SW_RST: c_int = 2;
pub const MT6795_TOPRGU_VENC_SW_RST: c_int = 3;
pub const MT6795_TOPRGU_VDEC_SW_RST: c_int = 4;
pub const MT6795_TOPRGU_IMG_SW_RST: c_int = 5;
pub const MT6795_TOPRGU_DDRPHY_SW_RST: c_int = 6;
pub const MT6795_TOPRGU_MD_SW_RST: c_int = 7;
pub const MT6795_TOPRGU_INFRA_AO_SW_RST: c_int = 8;
pub const MT6795_TOPRGU_MD_LITE_SW_RST: c_int = 9;
pub const MT6795_TOPRGU_APMIXED_SW_RST: c_int = 10;
pub const MT6795_TOPRGU_PWRAP_SPI_CTL_RST: c_int = 11;
pub const MT6795_TOPRGU_SW_RST_NUM: c_int = 12;
