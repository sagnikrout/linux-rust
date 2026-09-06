//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt7986-resets.h
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
// Author: Sam Shih <sam.shih@mediatek.com>
//
// INFRACFG resets
pub const MT7986_INFRACFG_PEXTP_MAC_SW_RST: c_int = 6;
pub const MT7986_INFRACFG_SSUSB_SW_RST: c_int = 7;
pub const MT7986_INFRACFG_EIP97_SW_RST: c_int = 8;
pub const MT7986_INFRACFG_AUDIO_SW_RST: c_int = 13;
pub const MT7986_INFRACFG_CQ_DMA_SW_RST: c_int = 14;
pub const MT7986_INFRACFG_TRNG_SW_RST: c_int = 17;
pub const MT7986_INFRACFG_AP_DMA_SW_RST: c_int = 32;
pub const MT7986_INFRACFG_I2C_SW_RST: c_int = 33;
pub const MT7986_INFRACFG_NFI_SW_RST: c_int = 34;
pub const MT7986_INFRACFG_SPI0_SW_RST: c_int = 35;
pub const MT7986_INFRACFG_SPI1_SW_RST: c_int = 36;
pub const MT7986_INFRACFG_UART0_SW_RST: c_int = 37;
pub const MT7986_INFRACFG_UART1_SW_RST: c_int = 38;
pub const MT7986_INFRACFG_UART2_SW_RST: c_int = 39;
pub const MT7986_INFRACFG_AUXADC_SW_RST: c_int = 43;
pub const MT7986_INFRACFG_APXGPT_SW_RST: c_int = 66;
pub const MT7986_INFRACFG_PWM_SW_RST: c_int = 68;
pub const MT7986_INFRACFG_SW_RST_NUM: c_int = 69;
// TOPRGU resets
pub const MT7986_TOPRGU_APMIXEDSYS_SW_RST: c_int = 0;
pub const MT7986_TOPRGU_SGMII0_SW_RST: c_int = 1;
pub const MT7986_TOPRGU_SGMII1_SW_RST: c_int = 2;
pub const MT7986_TOPRGU_INFRA_SW_RST: c_int = 3;
pub const MT7986_TOPRGU_U2PHY_SW_RST: c_int = 5;
pub const MT7986_TOPRGU_PCIE_SW_RST: c_int = 6;
pub const MT7986_TOPRGU_SSUSB_SW_RST: c_int = 7;
pub const MT7986_TOPRGU_ETHDMA_SW_RST: c_int = 20;
pub const MT7986_TOPRGU_CONSYS_SW_RST: c_int = 23;
pub const MT7986_TOPRGU_SW_RST_NUM: c_int = 24;
// ETHSYS Subsystem resets
pub const MT7986_ETHSYS_FE_SW_RST: c_int = 6;
pub const MT7986_ETHSYS_PMTR_SW_RST: c_int = 8;
pub const MT7986_ETHSYS_GMAC_SW_RST: c_int = 23;
pub const MT7986_ETHSYS_PPE0_SW_RST: c_int = 30;
pub const MT7986_ETHSYS_PPE1_SW_RST: c_int = 31;
pub const MT7986_ETHSYS_SW_RST_NUM: c_int = 32;
