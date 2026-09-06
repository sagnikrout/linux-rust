//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ingenic,jz4770-cgu.h
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
// This header provides clock numbers for the ingenic,jz4770-cgu DT binding.
//
pub const JZ4770_CLK_EXT: c_int = 0;
pub const JZ4770_CLK_OSC32K: c_int = 1;
pub const JZ4770_CLK_PLL0: c_int = 2;
pub const JZ4770_CLK_PLL1: c_int = 3;
pub const JZ4770_CLK_CCLK: c_int = 4;
pub const JZ4770_CLK_H0CLK: c_int = 5;
pub const JZ4770_CLK_H1CLK: c_int = 6;
pub const JZ4770_CLK_H2CLK: c_int = 7;
pub const JZ4770_CLK_C1CLK: c_int = 8;
pub const JZ4770_CLK_PCLK: c_int = 9;
pub const JZ4770_CLK_MMC0_MUX: c_int = 10;
pub const JZ4770_CLK_MMC0: c_int = 11;
pub const JZ4770_CLK_MMC1_MUX: c_int = 12;
pub const JZ4770_CLK_MMC1: c_int = 13;
pub const JZ4770_CLK_MMC2_MUX: c_int = 14;
pub const JZ4770_CLK_MMC2: c_int = 15;
pub const JZ4770_CLK_CIM: c_int = 16;
pub const JZ4770_CLK_UHC: c_int = 17;
pub const JZ4770_CLK_GPU: c_int = 18;
pub const JZ4770_CLK_BCH: c_int = 19;
pub const JZ4770_CLK_LPCLK_MUX: c_int = 20;
pub const JZ4770_CLK_GPS: c_int = 21;
pub const JZ4770_CLK_SSI_MUX: c_int = 22;
pub const JZ4770_CLK_PCM_MUX: c_int = 23;
pub const JZ4770_CLK_I2S: c_int = 24;
pub const JZ4770_CLK_OTG: c_int = 25;
pub const JZ4770_CLK_SSI0: c_int = 26;
pub const JZ4770_CLK_SSI1: c_int = 27;
pub const JZ4770_CLK_SSI2: c_int = 28;
pub const JZ4770_CLK_PCM0: c_int = 29;
pub const JZ4770_CLK_PCM1: c_int = 30;
pub const JZ4770_CLK_DMA: c_int = 31;
pub const JZ4770_CLK_I2C0: c_int = 32;
pub const JZ4770_CLK_I2C1: c_int = 33;
pub const JZ4770_CLK_I2C2: c_int = 34;
pub const JZ4770_CLK_UART0: c_int = 35;
pub const JZ4770_CLK_UART1: c_int = 36;
pub const JZ4770_CLK_UART2: c_int = 37;
pub const JZ4770_CLK_UART3: c_int = 38;
pub const JZ4770_CLK_IPU: c_int = 39;
pub const JZ4770_CLK_ADC: c_int = 40;
pub const JZ4770_CLK_AIC: c_int = 41;
pub const JZ4770_CLK_AUX: c_int = 42;
pub const JZ4770_CLK_VPU: c_int = 43;
pub const JZ4770_CLK_UHC_PHY: c_int = 44;
pub const JZ4770_CLK_OTG_PHY: c_int = 45;
pub const JZ4770_CLK_EXT512: c_int = 46;
pub const JZ4770_CLK_RTC: c_int = 47;
pub const JZ4770_CLK_BDMA: c_int = 48;
