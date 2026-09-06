//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt8195-resets.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Christine Zhu <christine.zhu@mediatek.com>
//
// TOPRGU resets
pub const MT8195_TOPRGU_CONN_MCU_SW_RST: c_int = 0;
pub const MT8195_TOPRGU_INFRA_GRST_SW_RST: c_int = 1;
pub const MT8195_TOPRGU_APU_SW_RST: c_int = 2;
pub const MT8195_TOPRGU_INFRA_AO_GRST_SW_RST: c_int = 6;
pub const MT8195_TOPRGU_MMSYS_SW_RST: c_int = 7;
pub const MT8195_TOPRGU_MFG_SW_RST: c_int = 8;
pub const MT8195_TOPRGU_VENC_SW_RST: c_int = 9;
pub const MT8195_TOPRGU_VDEC_SW_RST: c_int = 10;
pub const MT8195_TOPRGU_IMG_SW_RST: c_int = 11;
pub const MT8195_TOPRGU_APMIXEDSYS_SW_RST: c_int = 13;
pub const MT8195_TOPRGU_AUDIO_SW_RST: c_int = 14;
pub const MT8195_TOPRGU_CAMSYS_SW_RST: c_int = 15;
pub const MT8195_TOPRGU_EDPTX_SW_RST: c_int = 16;
pub const MT8195_TOPRGU_ADSPSYS_SW_RST: c_int = 21;
pub const MT8195_TOPRGU_DPTX_SW_RST: c_int = 22;
pub const MT8195_TOPRGU_SPMI_MST_SW_RST: c_int = 23;
pub const MT8195_TOPRGU_SW_RST_NUM: c_int = 16;
// INFRA resets
pub const MT8195_INFRA_RST0_THERM_CTRL_SWRST: c_int = 0;
pub const MT8195_INFRA_RST3_THERM_CTRL_PTP_SWRST: c_int = 1;
pub const MT8195_INFRA_RST4_THERM_CTRL_MCU_SWRST: c_int = 2;
pub const MT8195_INFRA_RST2_PCIE_P0_SWRST: c_int = 3;
pub const MT8195_INFRA_RST2_PCIE_P1_SWRST: c_int = 4;
pub const MT8195_INFRA_RST2_USBSIF_P1_SWRST: c_int = 5;
// VDOSYS1
pub const MT8195_VDOSYS1_SW0_RST_B_SMI_LARB2: c_int = 0;
pub const MT8195_VDOSYS1_SW0_RST_B_SMI_LARB3: c_int = 1;
pub const MT8195_VDOSYS1_SW0_RST_B_GALS: c_int = 2;
pub const MT8195_VDOSYS1_SW0_RST_B_FAKE_ENG0: c_int = 3;
pub const MT8195_VDOSYS1_SW0_RST_B_FAKE_ENG1: c_int = 4;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA0: c_int = 5;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA1: c_int = 6;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA2: c_int = 7;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA3: c_int = 8;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP_MERGE0: c_int = 9;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP_MERGE1: c_int = 10;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP_MERGE2: c_int = 11;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP_MERGE3: c_int = 12;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP_MERGE4: c_int = 13;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP2_TO_VDO1_DL_ASYNC: c_int = 14;
pub const MT8195_VDOSYS1_SW0_RST_B_VPP3_TO_VDO1_DL_ASYNC: c_int = 15;
pub const MT8195_VDOSYS1_SW0_RST_B_DISP_MUTEX: c_int = 16;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA4: c_int = 17;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA5: c_int = 18;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA6: c_int = 19;
pub const MT8195_VDOSYS1_SW0_RST_B_MDP_RDMA7: c_int = 20;
pub const MT8195_VDOSYS1_SW0_RST_B_DP_INTF0: c_int = 21;
pub const MT8195_VDOSYS1_SW0_RST_B_DPI0: c_int = 22;
pub const MT8195_VDOSYS1_SW0_RST_B_DPI1: c_int = 23;
pub const MT8195_VDOSYS1_SW0_RST_B_DISP_MONITOR: c_int = 24;
pub const MT8195_VDOSYS1_SW0_RST_B_MERGE0_DL_ASYNC: c_int = 25;
pub const MT8195_VDOSYS1_SW0_RST_B_MERGE1_DL_ASYNC: c_int = 26;
pub const MT8195_VDOSYS1_SW0_RST_B_MERGE2_DL_ASYNC: c_int = 27;
pub const MT8195_VDOSYS1_SW0_RST_B_MERGE3_DL_ASYNC: c_int = 28;
pub const MT8195_VDOSYS1_SW0_RST_B_MERGE4_DL_ASYNC: c_int = 29;
pub const MT8195_VDOSYS1_SW0_RST_B_VDO0_DSC_TO_VDO1_DL_ASYNC: c_int = 30;
pub const MT8195_VDOSYS1_SW0_RST_B_VDO0_MERGE_TO_VDO1_DL_ASYNC: c_int = 31;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_VDO_FE0: c_int = 32;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_GFX_FE0: c_int = 33;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_VDO_BE: c_int = 34;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_VDO_FE1: c_int = 48;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_GFX_FE1: c_int = 49;
pub const MT8195_VDOSYS1_SW1_RST_B_DISP_MIXER: c_int = 50;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_VDO_FE0_DL_ASYNC: c_int = 51;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_VDO_FE1_DL_ASYNC: c_int = 52;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_GFX_FE0_DL_ASYNC: c_int = 53;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_GFX_FE1_DL_ASYNC: c_int = 54;
pub const MT8195_VDOSYS1_SW1_RST_B_HDR_VDO_BE_DL_ASYNC: c_int = 55;
