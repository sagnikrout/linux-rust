//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt8188-resets.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Runyang Chen <runyang.chen@mediatek.com>
//
pub const MT8188_TOPRGU_CONN_MCU_SW_RST: c_int = 0;
pub const MT8188_TOPRGU_INFRA_GRST_SW_RST: c_int = 1;
pub const MT8188_TOPRGU_IPU0_SW_RST: c_int = 2;
pub const MT8188_TOPRGU_IPU1_SW_RST: c_int = 3;
pub const MT8188_TOPRGU_IPU2_SW_RST: c_int = 4;
pub const MT8188_TOPRGU_AUD_ASRC_SW_RST: c_int = 5;
pub const MT8188_TOPRGU_INFRA_SW_RST: c_int = 6;
pub const MT8188_TOPRGU_MMSYS_SW_RST: c_int = 7;
pub const MT8188_TOPRGU_MFG_SW_RST: c_int = 8;
pub const MT8188_TOPRGU_VENC_SW_RST: c_int = 9;
pub const MT8188_TOPRGU_VDEC_SW_RST: c_int = 10;
pub const MT8188_TOPRGU_CAM_VCORE_SW_RST: c_int = 11;
pub const MT8188_TOPRGU_SCP_SW_RST: c_int = 12;
pub const MT8188_TOPRGU_APMIXEDSYS_SW_RST: c_int = 13;
pub const MT8188_TOPRGU_AUDIO_SW_RST: c_int = 14;
pub const MT8188_TOPRGU_CAMSYS_SW_RST: c_int = 15;
pub const MT8188_TOPRGU_MJC_SW_RST: c_int = 16;
pub const MT8188_TOPRGU_PERI_SW_RST: c_int = 17;
pub const MT8188_TOPRGU_PERI_AO_SW_RST: c_int = 18;
pub const MT8188_TOPRGU_PCIE_SW_RST: c_int = 19;
pub const MT8188_TOPRGU_ADSPSYS_SW_RST: c_int = 21;
pub const MT8188_TOPRGU_DPTX_SW_RST: c_int = 22;
pub const MT8188_TOPRGU_SPMI_MST_SW_RST: c_int = 23;
pub const MT8188_TOPRGU_SW_RST_NUM: c_int = 24;
// INFRA resets
pub const MT8188_INFRA_RST1_THERMAL_MCU_RST: c_int = 0;
pub const MT8188_INFRA_RST1_THERMAL_CTRL_RST: c_int = 1;
pub const MT8188_INFRA_RST3_PTP_CTRL_RST: c_int = 2;
pub const MT8188_VDO0_RST_DISP_OVL0: c_int = 0;
pub const MT8188_VDO0_RST_FAKE_ENG0: c_int = 1;
pub const MT8188_VDO0_RST_DISP_CCORR0: c_int = 2;
pub const MT8188_VDO0_RST_DISP_MUTEX0: c_int = 3;
pub const MT8188_VDO0_RST_DISP_GAMMA0: c_int = 4;
pub const MT8188_VDO0_RST_DISP_DITHER0: c_int = 5;
pub const MT8188_VDO0_RST_DISP_WDMA0: c_int = 6;
pub const MT8188_VDO0_RST_DISP_RDMA0: c_int = 7;
pub const MT8188_VDO0_RST_DSI0: c_int = 8;
pub const MT8188_VDO0_RST_DSI1: c_int = 9;
pub const MT8188_VDO0_RST_DSC_WRAP0: c_int = 10;
pub const MT8188_VDO0_RST_VPP_MERGE0: c_int = 11;
pub const MT8188_VDO0_RST_DP_INTF0: c_int = 12;
pub const MT8188_VDO0_RST_DISP_AAL0: c_int = 13;
pub const MT8188_VDO0_RST_INLINEROT0: c_int = 14;
pub const MT8188_VDO0_RST_APB_BUS: c_int = 15;
pub const MT8188_VDO0_RST_DISP_COLOR0: c_int = 16;
pub const MT8188_VDO0_RST_MDP_WROT0: c_int = 17;
pub const MT8188_VDO0_RST_DISP_RSZ0: c_int = 18;
pub const MT8188_VDO1_RST_SMI_LARB2: c_int = 0;
pub const MT8188_VDO1_RST_SMI_LARB3: c_int = 1;
pub const MT8188_VDO1_RST_GALS: c_int = 2;
pub const MT8188_VDO1_RST_FAKE_ENG0: c_int = 3;
pub const MT8188_VDO1_RST_FAKE_ENG1: c_int = 4;
pub const MT8188_VDO1_RST_MDP_RDMA0: c_int = 5;
pub const MT8188_VDO1_RST_MDP_RDMA1: c_int = 6;
pub const MT8188_VDO1_RST_MDP_RDMA2: c_int = 7;
pub const MT8188_VDO1_RST_MDP_RDMA3: c_int = 8;
pub const MT8188_VDO1_RST_VPP_MERGE0: c_int = 9;
pub const MT8188_VDO1_RST_VPP_MERGE1: c_int = 10;
pub const MT8188_VDO1_RST_VPP_MERGE2: c_int = 11;
pub const MT8188_VDO1_RST_VPP_MERGE3: c_int = 12;
pub const MT8188_VDO1_RST_VPP_MERGE4: c_int = 13;
pub const MT8188_VDO1_RST_VPP2_TO_VDO1_DL_ASYNC: c_int = 14;
pub const MT8188_VDO1_RST_VPP3_TO_VDO1_DL_ASYNC: c_int = 15;
pub const MT8188_VDO1_RST_DISP_MUTEX: c_int = 16;
pub const MT8188_VDO1_RST_MDP_RDMA4: c_int = 17;
pub const MT8188_VDO1_RST_MDP_RDMA5: c_int = 18;
pub const MT8188_VDO1_RST_MDP_RDMA6: c_int = 19;
pub const MT8188_VDO1_RST_MDP_RDMA7: c_int = 20;
pub const MT8188_VDO1_RST_DP_INTF1_MMCK: c_int = 21;
pub const MT8188_VDO1_RST_DPI0_MM_CK: c_int = 22;
pub const MT8188_VDO1_RST_DPI1_MM_CK: c_int = 23;
pub const MT8188_VDO1_RST_MERGE0_DL_ASYNC: c_int = 24;
pub const MT8188_VDO1_RST_MERGE1_DL_ASYNC: c_int = 25;
pub const MT8188_VDO1_RST_MERGE2_DL_ASYNC: c_int = 26;
pub const MT8188_VDO1_RST_MERGE3_DL_ASYNC: c_int = 27;
pub const MT8188_VDO1_RST_MERGE4_DL_ASYNC: c_int = 28;
pub const MT8188_VDO1_RST_VDO0_DSC_TO_VDO1_DL_ASYNC: c_int = 29;
pub const MT8188_VDO1_RST_VDO0_MERGE_TO_VDO1_DL_ASYNC: c_int = 30;
pub const MT8188_VDO1_RST_PADDING0: c_int = 31;
pub const MT8188_VDO1_RST_PADDING1: c_int = 32;
pub const MT8188_VDO1_RST_PADDING2: c_int = 33;
pub const MT8188_VDO1_RST_PADDING3: c_int = 34;
pub const MT8188_VDO1_RST_PADDING4: c_int = 35;
pub const MT8188_VDO1_RST_PADDING5: c_int = 36;
pub const MT8188_VDO1_RST_PADDING6: c_int = 37;
pub const MT8188_VDO1_RST_PADDING7: c_int = 38;
pub const MT8188_VDO1_RST_DISP_RSZ0: c_int = 39;
pub const MT8188_VDO1_RST_DISP_RSZ1: c_int = 40;
pub const MT8188_VDO1_RST_DISP_RSZ2: c_int = 41;
pub const MT8188_VDO1_RST_DISP_RSZ3: c_int = 42;
pub const MT8188_VDO1_RST_HDR_VDO_FE0: c_int = 43;
pub const MT8188_VDO1_RST_HDR_GFX_FE0: c_int = 44;
pub const MT8188_VDO1_RST_HDR_VDO_BE: c_int = 45;
pub const MT8188_VDO1_RST_HDR_VDO_FE1: c_int = 46;
pub const MT8188_VDO1_RST_HDR_GFX_FE1: c_int = 47;
pub const MT8188_VDO1_RST_DISP_MIXER: c_int = 48;
pub const MT8188_VDO1_RST_HDR_VDO_FE0_DL_ASYNC: c_int = 49;
pub const MT8188_VDO1_RST_HDR_VDO_FE1_DL_ASYNC: c_int = 50;
pub const MT8188_VDO1_RST_HDR_GFX_FE0_DL_ASYNC: c_int = 51;
pub const MT8188_VDO1_RST_HDR_GFX_FE1_DL_ASYNC: c_int = 52;
pub const MT8188_VDO1_RST_HDR_VDO_BE_DL_ASYNC: c_int = 53;
