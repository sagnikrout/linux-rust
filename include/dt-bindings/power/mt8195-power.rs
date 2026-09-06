//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/mt8195-power.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
//
pub const MT8195_POWER_DOMAIN_PCIE_MAC_P0: c_int = 0;
pub const MT8195_POWER_DOMAIN_PCIE_MAC_P1: c_int = 1;
pub const MT8195_POWER_DOMAIN_PCIE_PHY: c_int = 2;
pub const MT8195_POWER_DOMAIN_SSUSB_PCIE_PHY: c_int = 3;
pub const MT8195_POWER_DOMAIN_CSI_RX_TOP: c_int = 4;
pub const MT8195_POWER_DOMAIN_ETHER: c_int = 5;
pub const MT8195_POWER_DOMAIN_ADSP: c_int = 6;
pub const MT8195_POWER_DOMAIN_AUDIO: c_int = 7;
pub const MT8195_POWER_DOMAIN_MFG0: c_int = 8;
pub const MT8195_POWER_DOMAIN_MFG1: c_int = 9;
pub const MT8195_POWER_DOMAIN_MFG2: c_int = 10;
pub const MT8195_POWER_DOMAIN_MFG3: c_int = 11;
pub const MT8195_POWER_DOMAIN_MFG4: c_int = 12;
pub const MT8195_POWER_DOMAIN_MFG5: c_int = 13;
pub const MT8195_POWER_DOMAIN_MFG6: c_int = 14;
pub const MT8195_POWER_DOMAIN_VPPSYS0: c_int = 15;
pub const MT8195_POWER_DOMAIN_VDOSYS0: c_int = 16;
pub const MT8195_POWER_DOMAIN_VPPSYS1: c_int = 17;
pub const MT8195_POWER_DOMAIN_VDOSYS1: c_int = 18;
pub const MT8195_POWER_DOMAIN_DP_TX: c_int = 19;
pub const MT8195_POWER_DOMAIN_EPD_TX: c_int = 20;
pub const MT8195_POWER_DOMAIN_HDMI_TX: c_int = 21;
pub const MT8195_POWER_DOMAIN_WPESYS: c_int = 22;
pub const MT8195_POWER_DOMAIN_VDEC0: c_int = 23;
pub const MT8195_POWER_DOMAIN_VDEC1: c_int = 24;
pub const MT8195_POWER_DOMAIN_VDEC2: c_int = 25;
pub const MT8195_POWER_DOMAIN_VENC: c_int = 26;
pub const MT8195_POWER_DOMAIN_VENC_CORE1: c_int = 27;
pub const MT8195_POWER_DOMAIN_IMG: c_int = 28;
pub const MT8195_POWER_DOMAIN_DIP: c_int = 29;
pub const MT8195_POWER_DOMAIN_IPE: c_int = 30;
pub const MT8195_POWER_DOMAIN_CAM: c_int = 31;
pub const MT8195_POWER_DOMAIN_CAM_RAWA: c_int = 32;
pub const MT8195_POWER_DOMAIN_CAM_RAWB: c_int = 33;
pub const MT8195_POWER_DOMAIN_CAM_MRAW: c_int = 34;
