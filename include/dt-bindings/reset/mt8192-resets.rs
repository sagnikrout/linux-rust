//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt8192-resets.h
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
// Author: Yong Liang <yong.liang@mediatek.com>
//
// TOPRGU resets
pub const MT8192_TOPRGU_MM_SW_RST: c_int = 1;
pub const MT8192_TOPRGU_MFG_SW_RST: c_int = 2;
pub const MT8192_TOPRGU_VENC_SW_RST: c_int = 3;
pub const MT8192_TOPRGU_VDEC_SW_RST: c_int = 4;
pub const MT8192_TOPRGU_IMG_SW_RST: c_int = 5;
pub const MT8192_TOPRGU_MD_SW_RST: c_int = 7;
pub const MT8192_TOPRGU_CONN_SW_RST: c_int = 9;
pub const MT8192_TOPRGU_CONN_MCU_SW_RST: c_int = 12;
pub const MT8192_TOPRGU_IPU0_SW_RST: c_int = 14;
pub const MT8192_TOPRGU_IPU1_SW_RST: c_int = 15;
pub const MT8192_TOPRGU_AUDIO_SW_RST: c_int = 17;
pub const MT8192_TOPRGU_CAMSYS_SW_RST: c_int = 18;
pub const MT8192_TOPRGU_MJC_SW_RST: c_int = 19;
pub const MT8192_TOPRGU_C2K_S2_SW_RST: c_int = 20;
pub const MT8192_TOPRGU_C2K_SW_RST: c_int = 21;
pub const MT8192_TOPRGU_PERI_SW_RST: c_int = 22;
pub const MT8192_TOPRGU_PERI_AO_SW_RST: c_int = 23;
pub const MT8192_TOPRGU_SW_RST_NUM: c_int = 23;
// MMSYS resets
pub const MT8192_MMSYS_SW0_RST_B_DISP_DSI0: c_int = 15;
// INFRA resets
pub const MT8192_INFRA_RST0_THERM_CTRL_SWRST: c_int = 0;
pub const MT8192_INFRA_RST2_PEXTP_PHY_SWRST: c_int = 1;
pub const MT8192_INFRA_RST3_THERM_CTRL_PTP_SWRST: c_int = 2;
pub const MT8192_INFRA_RST4_PCIE_TOP_SWRST: c_int = 3;
pub const MT8192_INFRA_RST4_THERM_CTRL_MCU_SWRST: c_int = 4;
