//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/mediatek,mt8196-power.h
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
// Copyright (c) 2025 Collabora Ltd
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
// SCPSYS Secure Power Manager - Direct Control
pub const MT8196_POWER_DOMAIN_MD: c_int = 0;
pub const MT8196_POWER_DOMAIN_CONN: c_int = 1;
pub const MT8196_POWER_DOMAIN_SSUSB_P0: c_int = 2;
pub const MT8196_POWER_DOMAIN_SSUSB_DP_PHY_P0: c_int = 3;
pub const MT8196_POWER_DOMAIN_SSUSB_P1: c_int = 4;
pub const MT8196_POWER_DOMAIN_SSUSB_P23: c_int = 5;
pub const MT8196_POWER_DOMAIN_SSUSB_PHY_P2: c_int = 6;
pub const MT8196_POWER_DOMAIN_PEXTP_MAC0: c_int = 7;
pub const MT8196_POWER_DOMAIN_PEXTP_MAC1: c_int = 8;
pub const MT8196_POWER_DOMAIN_PEXTP_MAC2: c_int = 9;
pub const MT8196_POWER_DOMAIN_PEXTP_PHY0: c_int = 10;
pub const MT8196_POWER_DOMAIN_PEXTP_PHY1: c_int = 11;
pub const MT8196_POWER_DOMAIN_PEXTP_PHY2: c_int = 12;
pub const MT8196_POWER_DOMAIN_AUDIO: c_int = 13;
pub const MT8196_POWER_DOMAIN_ADSP_TOP_DORMANT: c_int = 14;
pub const MT8196_POWER_DOMAIN_ADSP_INFRA: c_int = 15;
pub const MT8196_POWER_DOMAIN_ADSP_AO: c_int = 16;
// SCPSYS Secure Power Manager - HW Voter
pub const MT8196_POWER_DOMAIN_MM_PROC_DORMANT: c_int = 0;
pub const MT8196_POWER_DOMAIN_SSR: c_int = 1;
// HFRPSYS Multimedia Power Control (MMPC) - Direct Control
pub const MT8196_POWER_DOMAIN_EDPTX: c_int = 0;
pub const MT8196_POWER_DOMAIN_DPTX: c_int = 1;
// HFRPSYS MultiMedia Power Control (MMPC) - HW Voter
pub const MT8196_POWER_DOMAIN_VDE0: c_int = 0;
pub const MT8196_POWER_DOMAIN_VDE1: c_int = 1;
pub const MT8196_POWER_DOMAIN_VDE_VCORE0: c_int = 2;
pub const MT8196_POWER_DOMAIN_VEN0: c_int = 3;
pub const MT8196_POWER_DOMAIN_VEN1: c_int = 4;
pub const MT8196_POWER_DOMAIN_VEN2: c_int = 5;
pub const MT8196_POWER_DOMAIN_DISP_VCORE: c_int = 6;
pub const MT8196_POWER_DOMAIN_DIS0_DORMANT: c_int = 7;
pub const MT8196_POWER_DOMAIN_DIS1_DORMANT: c_int = 8;
pub const MT8196_POWER_DOMAIN_OVL0_DORMANT: c_int = 9;
pub const MT8196_POWER_DOMAIN_OVL1_DORMANT: c_int = 10;
pub const MT8196_POWER_DOMAIN_DISP_EDPTX_DORMANT: c_int = 11;
pub const MT8196_POWER_DOMAIN_DISP_DPTX_DORMANT: c_int = 12;
pub const MT8196_POWER_DOMAIN_MML0_SHUTDOWN: c_int = 13;
pub const MT8196_POWER_DOMAIN_MML1_SHUTDOWN: c_int = 14;
pub const MT8196_POWER_DOMAIN_MM_INFRA0: c_int = 15;
pub const MT8196_POWER_DOMAIN_MM_INFRA1: c_int = 16;
pub const MT8196_POWER_DOMAIN_MM_INFRA_AO: c_int = 17;
pub const MT8196_POWER_DOMAIN_CSI_BS_RX: c_int = 18;
pub const MT8196_POWER_DOMAIN_CSI_LS_RX: c_int = 19;
pub const MT8196_POWER_DOMAIN_DSI_PHY0: c_int = 20;
pub const MT8196_POWER_DOMAIN_DSI_PHY1: c_int = 21;
pub const MT8196_POWER_DOMAIN_DSI_PHY2: c_int = 22;
