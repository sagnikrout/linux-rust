//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt8186-resets.h
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
// TOPRGU resets
pub const MT8186_TOPRGU_INFRA_SW_RST: c_int = 0;
pub const MT8186_TOPRGU_MM_SW_RST: c_int = 1;
pub const MT8186_TOPRGU_MFG_SW_RST: c_int = 2;
pub const MT8186_TOPRGU_VENC_SW_RST: c_int = 3;
pub const MT8186_TOPRGU_VDEC_SW_RST: c_int = 4;
pub const MT8186_TOPRGU_IMG_SW_RST: c_int = 5;
pub const MT8186_TOPRGU_DDR_SW_RST: c_int = 6;
pub const MT8186_TOPRGU_INFRA_AO_SW_RST: c_int = 8;
pub const MT8186_TOPRGU_CONNSYS_SW_RST: c_int = 9;
pub const MT8186_TOPRGU_APMIXED_SW_RST: c_int = 10;
pub const MT8186_TOPRGU_PWRAP_SW_RST: c_int = 11;
pub const MT8186_TOPRGU_CONN_MCU_SW_RST: c_int = 12;
pub const MT8186_TOPRGU_IPNNA_SW_RST: c_int = 13;
pub const MT8186_TOPRGU_WPE_SW_RST: c_int = 14;
pub const MT8186_TOPRGU_ADSP_SW_RST: c_int = 15;
pub const MT8186_TOPRGU_AUDIO_SW_RST: c_int = 17;
pub const MT8186_TOPRGU_CAM_MAIN_SW_RST: c_int = 18;
pub const MT8186_TOPRGU_CAM_RAWA_SW_RST: c_int = 19;
pub const MT8186_TOPRGU_CAM_RAWB_SW_RST: c_int = 20;
pub const MT8186_TOPRGU_IPE_SW_RST: c_int = 21;
pub const MT8186_TOPRGU_IMG2_SW_RST: c_int = 22;
pub const MT8186_TOPRGU_SW_RST_NUM: c_int = 23;
// MMSYS resets
pub const MT8186_MMSYS_SW0_RST_B_DISP_DSI0: c_int = 19;
// INFRA resets
pub const MT8186_INFRA_THERMAL_CTRL_RST: c_int = 0;
pub const MT8186_INFRA_PTP_CTRL_RST: c_int = 1;
