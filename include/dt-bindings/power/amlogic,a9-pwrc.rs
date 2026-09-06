//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/amlogic,a9-pwrc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2026 Amlogic, Inc. All rights reserved
//
pub const PWRC_A9_DSPA_ID: c_int = 0;
pub const PWRC_A9_U3HSG_U3_ID: c_int = 1;
pub const PWRC_A9_DP_ID: c_int = 2;
pub const PWRC_A9_DOS_HCODEC_ID: c_int = 3;
pub const PWRC_A9_TAHOE_ID: c_int = 4;
pub const PWRC_A9_DOS_HEVC_ID: c_int = 5;
pub const PWRC_A9_U2H_ID: c_int = 6;
pub const PWRC_A9_U3DRD_B_ID: c_int = 7;
pub const PWRC_A9_VPU_HDMI_ID: c_int = 8;
pub const PWRC_A9_U2DRD_ID: c_int = 9;
pub const PWRC_A9_U3DRD_A_ID: c_int = 10;
pub const PWRC_A9_SD_EMMC_C_ID: c_int = 11;
pub const PWRC_A9_GE2D_ID: c_int = 12;
pub const PWRC_A9_AMFC_ID: c_int = 13;
pub const PWRC_A9_EDPTX_ID: c_int = 14;
pub const PWRC_A9_OPP_ID: c_int = 15;
pub const PWRC_A9_VICP_ID: c_int = 16;
pub const PWRC_A9_SD_EMMC_A_ID: c_int = 17;
pub const PWRC_A9_SD_EMMC_B_ID: c_int = 18;
pub const PWRC_A9_ETH_ID: c_int = 19;
pub const PWRC_A9_PCIE_A_ID: c_int = 20;
pub const PWRC_A9_PCIE_B_ID: c_int = 21;
pub const PWRC_A9_NNA_4T_ID: c_int = 22;
pub const PWRC_A9_HDMIRX_ID: c_int = 23;
pub const PWRC_A9_CVE_ID: c_int = 24;
pub const PWRC_A9_ISP_ID: c_int = 25;
pub const PWRC_A9_ETH_1G_ID: c_int = 26;
pub const PWRC_A9_U3HSG_HSG_ID: c_int = 27;
pub const PWRC_A9_U3DPPHY_U3_ID: c_int = 28;
pub const PWRC_A9_U3DPPHY_DP_ID: c_int = 29;
pub const PWRC_A9_PCIE3PHY_ID: c_int = 30;
pub const PWRC_A9_U3HSG_PCIE2_ID: c_int = 31;
pub const PWRC_A9_MALI_TOP_ID: c_int = 32;
pub const PWRC_A9_AO_SED_ID: c_int = 33;
pub const PWRC_A9_AO_IR_ID: c_int = 34;
pub const PWRC_A9_AO_UART_B_ID: c_int = 35;
pub const PWRC_A9_AO_UART_C_ID: c_int = 36;
pub const PWRC_A9_AO_UART_D_ID: c_int = 37;
pub const PWRC_A9_AO_SPISG_ID: c_int = 38;
pub const PWRC_A9_AO_UART_E_ID: c_int = 39;
pub const PWRC_A9_AO_CEC_ID: c_int = 40;
pub const PWRC_A9_EE_SRAMA_ID: c_int = 41;
pub const PWRC_A9_AUDIO_ID: c_int = 42;
pub const PWRC_A9_DMC0_ID: c_int = 43;
pub const PWRC_A9_GIC_ID: c_int = 44;
pub const PWRC_A9_DDRPHY_ID: c_int = 45;
pub const PWRC_A9_AUCPU_ID: c_int = 46;
pub const PWRC_A9_DSI0_ID: c_int = 47;
pub const PWRC_A9_DSI1_ID: c_int = 48;
pub const PWRC_A9_CAN0_ID: c_int = 49;
pub const PWRC_A9_CAN1_ID: c_int = 50;
