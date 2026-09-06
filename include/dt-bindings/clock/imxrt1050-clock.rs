//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imxrt1050-clock.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright(C) 2019
// Author(s): Giulio Benetti <giulio.benetti@benettiengineering.com>
//
pub const IMXRT1050_CLK_DUMMY: c_int = 0;
pub const IMXRT1050_CLK_CKIL: c_int = 1;
pub const IMXRT1050_CLK_CKIH: c_int = 2;
pub const IMXRT1050_CLK_OSC: c_int = 3;
pub const IMXRT1050_CLK_PLL2_PFD0_352M: c_int = 4;
pub const IMXRT1050_CLK_PLL2_PFD1_594M: c_int = 5;
pub const IMXRT1050_CLK_PLL2_PFD2_396M: c_int = 6;
pub const IMXRT1050_CLK_PLL3_PFD0_720M: c_int = 7;
pub const IMXRT1050_CLK_PLL3_PFD1_664_62M: c_int = 8;
pub const IMXRT1050_CLK_PLL3_PFD2_508_24M: c_int = 9;
pub const IMXRT1050_CLK_PLL3_PFD3_454_74M: c_int = 10;
pub const IMXRT1050_CLK_PLL2_198M: c_int = 11;
pub const IMXRT1050_CLK_PLL3_120M: c_int = 12;
pub const IMXRT1050_CLK_PLL3_80M: c_int = 13;
pub const IMXRT1050_CLK_PLL3_60M: c_int = 14;
pub const IMXRT1050_CLK_PLL1_BYPASS: c_int = 15;
pub const IMXRT1050_CLK_PLL2_BYPASS: c_int = 16;
pub const IMXRT1050_CLK_PLL3_BYPASS: c_int = 17;
pub const IMXRT1050_CLK_PLL5_BYPASS: c_int = 19;
pub const IMXRT1050_CLK_PLL1_REF_SEL: c_int = 20;
pub const IMXRT1050_CLK_PLL2_REF_SEL: c_int = 21;
pub const IMXRT1050_CLK_PLL3_REF_SEL: c_int = 22;
pub const IMXRT1050_CLK_PLL5_REF_SEL: c_int = 23;
pub const IMXRT1050_CLK_PRE_PERIPH_SEL: c_int = 24;
pub const IMXRT1050_CLK_PERIPH_SEL: c_int = 25;
pub const IMXRT1050_CLK_SEMC_ALT_SEL: c_int = 26;
pub const IMXRT1050_CLK_SEMC_SEL: c_int = 27;
pub const IMXRT1050_CLK_USDHC1_SEL: c_int = 28;
pub const IMXRT1050_CLK_USDHC2_SEL: c_int = 29;
pub const IMXRT1050_CLK_LPUART_SEL: c_int = 30;
pub const IMXRT1050_CLK_LCDIF_SEL: c_int = 31;
pub const IMXRT1050_CLK_VIDEO_POST_DIV_SEL: c_int = 32;
pub const IMXRT1050_CLK_VIDEO_DIV: c_int = 33;
pub const IMXRT1050_CLK_ARM_PODF: c_int = 34;
pub const IMXRT1050_CLK_LPUART_PODF: c_int = 35;
pub const IMXRT1050_CLK_USDHC1_PODF: c_int = 36;
pub const IMXRT1050_CLK_USDHC2_PODF: c_int = 37;
pub const IMXRT1050_CLK_SEMC_PODF: c_int = 38;
pub const IMXRT1050_CLK_AHB_PODF: c_int = 39;
pub const IMXRT1050_CLK_LCDIF_PRED: c_int = 40;
pub const IMXRT1050_CLK_LCDIF_PODF: c_int = 41;
pub const IMXRT1050_CLK_USDHC1: c_int = 42;
pub const IMXRT1050_CLK_USDHC2: c_int = 43;
pub const IMXRT1050_CLK_LPUART1: c_int = 44;
pub const IMXRT1050_CLK_SEMC: c_int = 45;
pub const IMXRT1050_CLK_LCDIF_APB: c_int = 46;
pub const IMXRT1050_CLK_PLL1_ARM: c_int = 47;
pub const IMXRT1050_CLK_PLL2_SYS: c_int = 48;
pub const IMXRT1050_CLK_PLL3_USB_OTG: c_int = 49;
pub const IMXRT1050_CLK_PLL4_AUDIO: c_int = 50;
pub const IMXRT1050_CLK_PLL5_VIDEO: c_int = 51;
pub const IMXRT1050_CLK_PLL6_ENET: c_int = 52;
pub const IMXRT1050_CLK_PLL7_USB_HOST: c_int = 53;
pub const IMXRT1050_CLK_LCDIF_PIX: c_int = 54;
pub const IMXRT1050_CLK_USBOH3: c_int = 55;
pub const IMXRT1050_CLK_IPG_PDOF: c_int = 56;
pub const IMXRT1050_CLK_PER_CLK_SEL: c_int = 57;
pub const IMXRT1050_CLK_PER_PDOF: c_int = 58;
pub const IMXRT1050_CLK_DMA: c_int = 59;
pub const IMXRT1050_CLK_DMA_MUX: c_int = 60;
pub const IMXRT1050_CLK_END: c_int = 61;
