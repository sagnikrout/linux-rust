//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun5i-ccu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//
pub const CLK_HOSC: c_int = 1;
pub const CLK_PLL_VIDEO0_2X: c_int = 9;
pub const CLK_PLL_VIDEO1_2X: c_int = 16;
pub const CLK_CPU: c_int = 17;
pub const CLK_AHB_OTG: c_int = 23;
pub const CLK_AHB_EHCI: c_int = 24;
pub const CLK_AHB_OHCI: c_int = 25;
pub const CLK_AHB_SS: c_int = 26;
pub const CLK_AHB_DMA: c_int = 27;
pub const CLK_AHB_BIST: c_int = 28;
pub const CLK_AHB_MMC0: c_int = 29;
pub const CLK_AHB_MMC1: c_int = 30;
pub const CLK_AHB_MMC2: c_int = 31;
pub const CLK_AHB_NAND: c_int = 32;
pub const CLK_AHB_SDRAM: c_int = 33;
pub const CLK_AHB_EMAC: c_int = 34;
pub const CLK_AHB_TS: c_int = 35;
pub const CLK_AHB_SPI0: c_int = 36;
pub const CLK_AHB_SPI1: c_int = 37;
pub const CLK_AHB_SPI2: c_int = 38;
pub const CLK_AHB_GPS: c_int = 39;
pub const CLK_AHB_HSTIMER: c_int = 40;
pub const CLK_AHB_VE: c_int = 41;
pub const CLK_AHB_TVE: c_int = 42;
pub const CLK_AHB_LCD: c_int = 43;
pub const CLK_AHB_CSI: c_int = 44;
pub const CLK_AHB_HDMI: c_int = 45;
pub const CLK_AHB_DE_BE: c_int = 46;
pub const CLK_AHB_DE_FE: c_int = 47;
pub const CLK_AHB_IEP: c_int = 48;
pub const CLK_AHB_GPU: c_int = 49;
pub const CLK_APB0_CODEC: c_int = 50;
pub const CLK_APB0_SPDIF: c_int = 51;
pub const CLK_APB0_I2S: c_int = 52;
pub const CLK_APB0_PIO: c_int = 53;
pub const CLK_APB0_IR: c_int = 54;
pub const CLK_APB0_KEYPAD: c_int = 55;
pub const CLK_APB1_I2C0: c_int = 56;
pub const CLK_APB1_I2C1: c_int = 57;
pub const CLK_APB1_I2C2: c_int = 58;
pub const CLK_APB1_UART0: c_int = 59;
pub const CLK_APB1_UART1: c_int = 60;
pub const CLK_APB1_UART2: c_int = 61;
pub const CLK_APB1_UART3: c_int = 62;
pub const CLK_NAND: c_int = 63;
pub const CLK_MMC0: c_int = 64;
pub const CLK_MMC1: c_int = 65;
pub const CLK_MMC2: c_int = 66;
pub const CLK_TS: c_int = 67;
pub const CLK_SS: c_int = 68;
pub const CLK_SPI0: c_int = 69;
pub const CLK_SPI1: c_int = 70;
pub const CLK_SPI2: c_int = 71;
pub const CLK_IR: c_int = 72;
pub const CLK_I2S: c_int = 73;
pub const CLK_SPDIF: c_int = 74;
pub const CLK_KEYPAD: c_int = 75;
pub const CLK_USB_OHCI: c_int = 76;
pub const CLK_USB_PHY0: c_int = 77;
pub const CLK_USB_PHY1: c_int = 78;
pub const CLK_GPS: c_int = 79;
pub const CLK_DRAM_VE: c_int = 80;
pub const CLK_DRAM_CSI: c_int = 81;
pub const CLK_DRAM_TS: c_int = 82;
pub const CLK_DRAM_TVE: c_int = 83;
pub const CLK_DRAM_DE_FE: c_int = 84;
pub const CLK_DRAM_DE_BE: c_int = 85;
pub const CLK_DRAM_ACE: c_int = 86;
pub const CLK_DRAM_IEP: c_int = 87;
pub const CLK_DE_BE: c_int = 88;
pub const CLK_DE_FE: c_int = 89;
pub const CLK_TCON_CH0: c_int = 90;
pub const CLK_TCON_CH1: c_int = 92;
pub const CLK_CSI: c_int = 93;
pub const CLK_VE: c_int = 94;
pub const CLK_CODEC: c_int = 95;
pub const CLK_AVS: c_int = 96;
pub const CLK_HDMI: c_int = 97;
pub const CLK_GPU: c_int = 98;
pub const CLK_MBUS: c_int = 99;
pub const CLK_IEP: c_int = 100;
