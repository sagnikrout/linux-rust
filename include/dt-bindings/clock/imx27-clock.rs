//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx27-clock.h
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
// Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
//
pub const IMX27_CLK_DUMMY: c_int = 0;
pub const IMX27_CLK_CKIH: c_int = 1;
pub const IMX27_CLK_CKIL: c_int = 2;
pub const IMX27_CLK_MPLL: c_int = 3;
pub const IMX27_CLK_SPLL: c_int = 4;
pub const IMX27_CLK_MPLL_MAIN2: c_int = 5;
pub const IMX27_CLK_AHB: c_int = 6;
pub const IMX27_CLK_IPG: c_int = 7;
pub const IMX27_CLK_NFC_DIV: c_int = 8;
pub const IMX27_CLK_PER1_DIV: c_int = 9;
pub const IMX27_CLK_PER2_DIV: c_int = 10;
pub const IMX27_CLK_PER3_DIV: c_int = 11;
pub const IMX27_CLK_PER4_DIV: c_int = 12;
pub const IMX27_CLK_VPU_SEL: c_int = 13;
pub const IMX27_CLK_VPU_DIV: c_int = 14;
pub const IMX27_CLK_USB_DIV: c_int = 15;
pub const IMX27_CLK_CPU_SEL: c_int = 16;
pub const IMX27_CLK_CLKO_SEL: c_int = 17;
pub const IMX27_CLK_CPU_DIV: c_int = 18;
pub const IMX27_CLK_CLKO_DIV: c_int = 19;
pub const IMX27_CLK_SSI1_SEL: c_int = 20;
pub const IMX27_CLK_SSI2_SEL: c_int = 21;
pub const IMX27_CLK_SSI1_DIV: c_int = 22;
pub const IMX27_CLK_SSI2_DIV: c_int = 23;
pub const IMX27_CLK_CLKO_EN: c_int = 24;
pub const IMX27_CLK_SSI2_IPG_GATE: c_int = 25;
pub const IMX27_CLK_SSI1_IPG_GATE: c_int = 26;
pub const IMX27_CLK_SLCDC_IPG_GATE: c_int = 27;
pub const IMX27_CLK_SDHC3_IPG_GATE: c_int = 28;
pub const IMX27_CLK_SDHC2_IPG_GATE: c_int = 29;
pub const IMX27_CLK_SDHC1_IPG_GATE: c_int = 30;
pub const IMX27_CLK_SCC_IPG_GATE: c_int = 31;
pub const IMX27_CLK_SAHARA_IPG_GATE: c_int = 32;
pub const IMX27_CLK_RTC_IPG_GATE: c_int = 33;
pub const IMX27_CLK_PWM_IPG_GATE: c_int = 34;
pub const IMX27_CLK_OWIRE_IPG_GATE: c_int = 35;
pub const IMX27_CLK_LCDC_IPG_GATE: c_int = 36;
pub const IMX27_CLK_KPP_IPG_GATE: c_int = 37;
pub const IMX27_CLK_IIM_IPG_GATE: c_int = 38;
pub const IMX27_CLK_I2C2_IPG_GATE: c_int = 39;
pub const IMX27_CLK_I2C1_IPG_GATE: c_int = 40;
pub const IMX27_CLK_GPT6_IPG_GATE: c_int = 41;
pub const IMX27_CLK_GPT5_IPG_GATE: c_int = 42;
pub const IMX27_CLK_GPT4_IPG_GATE: c_int = 43;
pub const IMX27_CLK_GPT3_IPG_GATE: c_int = 44;
pub const IMX27_CLK_GPT2_IPG_GATE: c_int = 45;
pub const IMX27_CLK_GPT1_IPG_GATE: c_int = 46;
pub const IMX27_CLK_GPIO_IPG_GATE: c_int = 47;
pub const IMX27_CLK_FEC_IPG_GATE: c_int = 48;
pub const IMX27_CLK_EMMA_IPG_GATE: c_int = 49;
pub const IMX27_CLK_DMA_IPG_GATE: c_int = 50;
pub const IMX27_CLK_CSPI3_IPG_GATE: c_int = 51;
pub const IMX27_CLK_CSPI2_IPG_GATE: c_int = 52;
pub const IMX27_CLK_CSPI1_IPG_GATE: c_int = 53;
pub const IMX27_CLK_NFC_BAUD_GATE: c_int = 54;
pub const IMX27_CLK_SSI2_BAUD_GATE: c_int = 55;
pub const IMX27_CLK_SSI1_BAUD_GATE: c_int = 56;
pub const IMX27_CLK_VPU_BAUD_GATE: c_int = 57;
pub const IMX27_CLK_PER4_GATE: c_int = 58;
pub const IMX27_CLK_PER3_GATE: c_int = 59;
pub const IMX27_CLK_PER2_GATE: c_int = 60;
pub const IMX27_CLK_PER1_GATE: c_int = 61;
pub const IMX27_CLK_USB_AHB_GATE: c_int = 62;
pub const IMX27_CLK_SLCDC_AHB_GATE: c_int = 63;
pub const IMX27_CLK_SAHARA_AHB_GATE: c_int = 64;
pub const IMX27_CLK_LCDC_AHB_GATE: c_int = 65;
pub const IMX27_CLK_VPU_AHB_GATE: c_int = 66;
pub const IMX27_CLK_FEC_AHB_GATE: c_int = 67;
pub const IMX27_CLK_EMMA_AHB_GATE: c_int = 68;
pub const IMX27_CLK_EMI_AHB_GATE: c_int = 69;
pub const IMX27_CLK_DMA_AHB_GATE: c_int = 70;
pub const IMX27_CLK_CSI_AHB_GATE: c_int = 71;
pub const IMX27_CLK_BROM_AHB_GATE: c_int = 72;
pub const IMX27_CLK_ATA_AHB_GATE: c_int = 73;
pub const IMX27_CLK_WDOG_IPG_GATE: c_int = 74;
pub const IMX27_CLK_USB_IPG_GATE: c_int = 75;
pub const IMX27_CLK_UART6_IPG_GATE: c_int = 76;
pub const IMX27_CLK_UART5_IPG_GATE: c_int = 77;
pub const IMX27_CLK_UART4_IPG_GATE: c_int = 78;
pub const IMX27_CLK_UART3_IPG_GATE: c_int = 79;
pub const IMX27_CLK_UART2_IPG_GATE: c_int = 80;
pub const IMX27_CLK_UART1_IPG_GATE: c_int = 81;
pub const IMX27_CLK_CKIH_DIV1P5: c_int = 82;
pub const IMX27_CLK_FPM: c_int = 83;
pub const IMX27_CLK_MPLL_OSC_SEL: c_int = 84;
pub const IMX27_CLK_MPLL_SEL: c_int = 85;
pub const IMX27_CLK_SPLL_GATE: c_int = 86;
pub const IMX27_CLK_MSHC_DIV: c_int = 87;
pub const IMX27_CLK_RTIC_IPG_GATE: c_int = 88;
pub const IMX27_CLK_MSHC_IPG_GATE: c_int = 89;
pub const IMX27_CLK_RTIC_AHB_GATE: c_int = 90;
pub const IMX27_CLK_MSHC_BAUD_GATE: c_int = 91;
pub const IMX27_CLK_CKIH_GATE: c_int = 92;
pub const IMX27_CLK_MAX: c_int = 93;
