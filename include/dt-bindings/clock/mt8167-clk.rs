//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mt8167-clk.h
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
// Copyright (c) 2020 MediaTek Inc.
// Copyright (c) 2020 BayLibre, SAS.
// Author: James Liao <jamesjj.liao@mediatek.com>
// Fabien Parent <fparent@baylibre.com>
//
// MT8167 is based on MT8516

// APMIXEDSYS

// TOPCKGEN

// MFGCFG
pub const CLK_MFG_BAXI: c_int = 0;
pub const CLK_MFG_BMEM: c_int = 1;
pub const CLK_MFG_BG3D: c_int = 2;
pub const CLK_MFG_B26M: c_int = 3;
pub const CLK_MFG_NR_CLK: c_int = 4;
// MMSYS
pub const CLK_MM_SMI_COMMON: c_int = 0;
pub const CLK_MM_SMI_LARB0: c_int = 1;
pub const CLK_MM_CAM_MDP: c_int = 2;
pub const CLK_MM_MDP_RDMA: c_int = 3;
pub const CLK_MM_MDP_RSZ0: c_int = 4;
pub const CLK_MM_MDP_RSZ1: c_int = 5;
pub const CLK_MM_MDP_TDSHP: c_int = 6;
pub const CLK_MM_MDP_WDMA: c_int = 7;
pub const CLK_MM_MDP_WROT: c_int = 8;
pub const CLK_MM_FAKE_ENG: c_int = 9;
pub const CLK_MM_DISP_OVL0: c_int = 10;
pub const CLK_MM_DISP_RDMA0: c_int = 11;
pub const CLK_MM_DISP_RDMA1: c_int = 12;
pub const CLK_MM_DISP_WDMA: c_int = 13;
pub const CLK_MM_DISP_COLOR: c_int = 14;
pub const CLK_MM_DISP_CCORR: c_int = 15;
pub const CLK_MM_DISP_AAL: c_int = 16;
pub const CLK_MM_DISP_GAMMA: c_int = 17;
pub const CLK_MM_DISP_DITHER: c_int = 18;
pub const CLK_MM_DISP_UFOE: c_int = 19;
pub const CLK_MM_DISP_PWM_MM: c_int = 20;
pub const CLK_MM_DISP_PWM_26M: c_int = 21;
pub const CLK_MM_DSI_ENGINE: c_int = 22;
pub const CLK_MM_DSI_DIGITAL: c_int = 23;
pub const CLK_MM_DPI0_ENGINE: c_int = 24;
pub const CLK_MM_DPI0_PXL: c_int = 25;
pub const CLK_MM_LVDS_PXL: c_int = 26;
pub const CLK_MM_LVDS_CTS: c_int = 27;
pub const CLK_MM_DPI1_ENGINE: c_int = 28;
pub const CLK_MM_DPI1_PXL: c_int = 29;
pub const CLK_MM_HDMI_PXL: c_int = 30;
pub const CLK_MM_HDMI_SPDIF: c_int = 31;
pub const CLK_MM_HDMI_ADSP_BCK: c_int = 32;
pub const CLK_MM_HDMI_PLL: c_int = 33;
pub const CLK_MM_NR_CLK: c_int = 34;
// IMGSYS
pub const CLK_IMG_LARB1_SMI: c_int = 0;
pub const CLK_IMG_CAM_SMI: c_int = 1;
pub const CLK_IMG_CAM_CAM: c_int = 2;
pub const CLK_IMG_SEN_TG: c_int = 3;
pub const CLK_IMG_SEN_CAM: c_int = 4;
pub const CLK_IMG_VENC: c_int = 5;
pub const CLK_IMG_NR_CLK: c_int = 6;
// VDECSYS
pub const CLK_VDEC_CKEN: c_int = 0;
pub const CLK_VDEC_LARB1_CKEN: c_int = 1;
pub const CLK_VDEC_NR_CLK: c_int = 2;
