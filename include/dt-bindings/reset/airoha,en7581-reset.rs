//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/airoha,en7581-reset.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2024 AIROHA Inc
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
//
// RST_CTRL2
pub const EN7581_XPON_PHY_RST: c_int = 0;
pub const EN7581_CPU_TIMER2_RST: c_int = 1;
pub const EN7581_HSUART_RST: c_int = 2;
pub const EN7581_UART4_RST: c_int = 3;
pub const EN7581_UART5_RST: c_int = 4;
pub const EN7581_I2C2_RST: c_int = 5;
pub const EN7581_XSI_MAC_RST: c_int = 6;
pub const EN7581_XSI_PHY_RST: c_int = 7;
pub const EN7581_NPU_RST: c_int = 8;
pub const EN7581_I2S_RST: c_int = 9;
pub const EN7581_TRNG_RST: c_int = 10;
pub const EN7581_TRNG_MSTART_RST: c_int = 11;
pub const EN7581_DUAL_HSI0_RST: c_int = 12;
pub const EN7581_DUAL_HSI1_RST: c_int = 13;
pub const EN7581_HSI_RST: c_int = 14;
pub const EN7581_DUAL_HSI0_MAC_RST: c_int = 15;
pub const EN7581_DUAL_HSI1_MAC_RST: c_int = 16;
pub const EN7581_HSI_MAC_RST: c_int = 17;
pub const EN7581_WDMA_RST: c_int = 18;
pub const EN7581_WOE0_RST: c_int = 19;
pub const EN7581_WOE1_RST: c_int = 20;
pub const EN7581_HSDMA_RST: c_int = 21;
pub const EN7581_TDMA_RST: c_int = 22;
pub const EN7581_EMMC_RST: c_int = 23;
pub const EN7581_SOE_RST: c_int = 24;
pub const EN7581_PCIE2_RST: c_int = 25;
pub const EN7581_XFP_MAC_RST: c_int = 26;
pub const EN7581_USB_HOST_P1_RST: c_int = 27;
pub const EN7581_USB_HOST_P1_U3_PHY_RST: c_int = 28;
// RST_CTRL1
pub const EN7581_PCM1_ZSI_ISI_RST: c_int = 29;
pub const EN7581_FE_PDMA_RST: c_int = 30;
pub const EN7581_FE_QDMA_RST: c_int = 31;
pub const EN7581_PCM_SPIWP_RST: c_int = 32;
pub const EN7581_CRYPTO_RST: c_int = 33;
pub const EN7581_TIMER_RST: c_int = 34;
pub const EN7581_PCM1_RST: c_int = 35;
pub const EN7581_UART_RST: c_int = 36;
pub const EN7581_GPIO_RST: c_int = 37;
pub const EN7581_GDMA_RST: c_int = 38;
pub const EN7581_I2C_MASTER_RST: c_int = 39;
pub const EN7581_PCM2_ZSI_ISI_RST: c_int = 40;
pub const EN7581_SFC_RST: c_int = 41;
pub const EN7581_UART2_RST: c_int = 42;
pub const EN7581_GDMP_RST: c_int = 43;
pub const EN7581_FE_RST: c_int = 44;
pub const EN7581_USB_HOST_P0_RST: c_int = 45;
pub const EN7581_GSW_RST: c_int = 46;
pub const EN7581_SFC2_PCM_RST: c_int = 47;
pub const EN7581_PCIE0_RST: c_int = 48;
pub const EN7581_PCIE1_RST: c_int = 49;
pub const EN7581_CPU_TIMER_RST: c_int = 50;
pub const EN7581_PCIE_HB_RST: c_int = 51;
pub const EN7581_XPON_MAC_RST: c_int = 52;
// RST_PCIC
pub const EN7581_PCIC_PERSTOUT0_RST: c_int = 53;
pub const EN7581_PCIC_PERSTOUT1_RST: c_int = 54;
pub const EN7581_PCIC_PERSTOUT2_RST: c_int = 55;
