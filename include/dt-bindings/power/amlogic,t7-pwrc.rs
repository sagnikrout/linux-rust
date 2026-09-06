//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/amlogic,t7-pwrc.h
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
// Copyright (c) 2023 Amlogic, Inc.
// Author: Hongyu Chen <hongyu.chen1@amlogic.com>
//
pub const PWRC_T7_DSPA_ID: c_int = 0;
pub const PWRC_T7_DSPB_ID: c_int = 1;
pub const PWRC_T7_DOS_HCODEC_ID: c_int = 2;
pub const PWRC_T7_DOS_HEVC_ID: c_int = 3;
pub const PWRC_T7_DOS_VDEC_ID: c_int = 4;
pub const PWRC_T7_DOS_WAVE_ID: c_int = 5;
pub const PWRC_T7_VPU_HDMI_ID: c_int = 6;
pub const PWRC_T7_USB_COMB_ID: c_int = 7;
pub const PWRC_T7_PCIE_ID: c_int = 8;
pub const PWRC_T7_GE2D_ID: c_int = 9;
pub const PWRC_T7_SRAMA_ID: c_int = 10;
pub const PWRC_T7_SRAMB_ID: c_int = 11;
pub const PWRC_T7_HDMIRX_ID: c_int = 12;
pub const PWRC_T7_VI_CLK1_ID: c_int = 13;
pub const PWRC_T7_VI_CLK2_ID: c_int = 14;
pub const PWRC_T7_ETH_ID: c_int = 15;
pub const PWRC_T7_ISP_ID: c_int = 16;
pub const PWRC_T7_MIPI_ISP_ID: c_int = 17;
pub const PWRC_T7_GDC_ID: c_int = 18;
pub const PWRC_T7_CVE_ID: c_int = 18;
pub const PWRC_T7_DEWARP_ID: c_int = 19;
pub const PWRC_T7_SDIO_A_ID: c_int = 20;
pub const PWRC_T7_SDIO_B_ID: c_int = 21;
pub const PWRC_T7_EMMC_ID: c_int = 22;
pub const PWRC_T7_MALI_SC0_ID: c_int = 23;
pub const PWRC_T7_MALI_SC1_ID: c_int = 24;
pub const PWRC_T7_MALI_SC2_ID: c_int = 25;
pub const PWRC_T7_MALI_SC3_ID: c_int = 26;
pub const PWRC_T7_MALI_TOP_ID: c_int = 27;
pub const PWRC_T7_NNA_CORE0_ID: c_int = 28;
pub const PWRC_T7_NNA_CORE1_ID: c_int = 29;
pub const PWRC_T7_NNA_CORE2_ID: c_int = 30;
pub const PWRC_T7_NNA_CORE3_ID: c_int = 31;
pub const PWRC_T7_NNA_TOP_ID: c_int = 32;
pub const PWRC_T7_DDR0_ID: c_int = 33;
pub const PWRC_T7_DDR1_ID: c_int = 34;
pub const PWRC_T7_DMC0_ID: c_int = 35;
pub const PWRC_T7_DMC1_ID: c_int = 36;
pub const PWRC_T7_NOC_ID: c_int = 37;
pub const PWRC_T7_NIC2_ID: c_int = 38;
pub const PWRC_T7_NIC3_ID: c_int = 39;
pub const PWRC_T7_CCI_ID: c_int = 40;
pub const PWRC_T7_MIPI_DSI0_ID: c_int = 41;
pub const PWRC_T7_SPICC0_ID: c_int = 42;
pub const PWRC_T7_SPICC1_ID: c_int = 43;
pub const PWRC_T7_SPICC2_ID: c_int = 44;
pub const PWRC_T7_SPICC3_ID: c_int = 45;
pub const PWRC_T7_SPICC4_ID: c_int = 46;
pub const PWRC_T7_SPICC5_ID: c_int = 47;
pub const PWRC_T7_EDP0_ID: c_int = 48;
pub const PWRC_T7_EDP1_ID: c_int = 49;
pub const PWRC_T7_MIPI_DSI1_ID: c_int = 50;
pub const PWRC_T7_AUDIO_ID: c_int = 51;
