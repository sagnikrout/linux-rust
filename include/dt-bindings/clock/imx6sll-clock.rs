//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx6sll-clock.h
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
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017-2018 NXP.
//
pub const IMX6SLL_CLK_DUMMY: c_int = 0;
pub const IMX6SLL_CLK_CKIL: c_int = 1;
pub const IMX6SLL_CLK_OSC: c_int = 2;
pub const IMX6SLL_PLL1_BYPASS_SRC: c_int = 3;
pub const IMX6SLL_PLL2_BYPASS_SRC: c_int = 4;
pub const IMX6SLL_PLL3_BYPASS_SRC: c_int = 5;
pub const IMX6SLL_PLL4_BYPASS_SRC: c_int = 6;
pub const IMX6SLL_PLL5_BYPASS_SRC: c_int = 7;
pub const IMX6SLL_PLL6_BYPASS_SRC: c_int = 8;
pub const IMX6SLL_PLL7_BYPASS_SRC: c_int = 9;
pub const IMX6SLL_CLK_PLL1: c_int = 10;
pub const IMX6SLL_CLK_PLL2: c_int = 11;
pub const IMX6SLL_CLK_PLL3: c_int = 12;
pub const IMX6SLL_CLK_PLL4: c_int = 13;
pub const IMX6SLL_CLK_PLL5: c_int = 14;
pub const IMX6SLL_CLK_PLL6: c_int = 15;
pub const IMX6SLL_CLK_PLL7: c_int = 16;
pub const IMX6SLL_PLL1_BYPASS: c_int = 17;
pub const IMX6SLL_PLL2_BYPASS: c_int = 18;
pub const IMX6SLL_PLL3_BYPASS: c_int = 19;
pub const IMX6SLL_PLL4_BYPASS: c_int = 20;
pub const IMX6SLL_PLL5_BYPASS: c_int = 21;
pub const IMX6SLL_PLL6_BYPASS: c_int = 22;
pub const IMX6SLL_PLL7_BYPASS: c_int = 23;
pub const IMX6SLL_CLK_PLL1_SYS: c_int = 24;
pub const IMX6SLL_CLK_PLL2_BUS: c_int = 25;
pub const IMX6SLL_CLK_PLL3_USB_OTG: c_int = 26;
pub const IMX6SLL_CLK_PLL4_AUDIO: c_int = 27;
pub const IMX6SLL_CLK_PLL5_VIDEO: c_int = 28;
pub const IMX6SLL_CLK_PLL6_ENET: c_int = 29;
pub const IMX6SLL_CLK_PLL7_USB_HOST: c_int = 30;
pub const IMX6SLL_CLK_USBPHY1: c_int = 31;
pub const IMX6SLL_CLK_USBPHY2: c_int = 32;
pub const IMX6SLL_CLK_USBPHY1_GATE: c_int = 33;
pub const IMX6SLL_CLK_USBPHY2_GATE: c_int = 34;
pub const IMX6SLL_CLK_PLL2_PFD0: c_int = 35;
pub const IMX6SLL_CLK_PLL2_PFD1: c_int = 36;
pub const IMX6SLL_CLK_PLL2_PFD2: c_int = 37;
pub const IMX6SLL_CLK_PLL2_PFD3: c_int = 38;
pub const IMX6SLL_CLK_PLL3_PFD0: c_int = 39;
pub const IMX6SLL_CLK_PLL3_PFD1: c_int = 40;
pub const IMX6SLL_CLK_PLL3_PFD2: c_int = 41;
pub const IMX6SLL_CLK_PLL3_PFD3: c_int = 42;
pub const IMX6SLL_CLK_PLL4_POST_DIV: c_int = 43;
pub const IMX6SLL_CLK_PLL4_AUDIO_DIV: c_int = 44;
pub const IMX6SLL_CLK_PLL5_POST_DIV: c_int = 45;
pub const IMX6SLL_CLK_PLL5_VIDEO_DIV: c_int = 46;
pub const IMX6SLL_CLK_PLL2_198M: c_int = 47;
pub const IMX6SLL_CLK_PLL3_120M: c_int = 48;
pub const IMX6SLL_CLK_PLL3_80M: c_int = 49;
pub const IMX6SLL_CLK_PLL3_60M: c_int = 50;
pub const IMX6SLL_CLK_STEP: c_int = 51;
pub const IMX6SLL_CLK_PLL1_SW: c_int = 52;
pub const IMX6SLL_CLK_AXI_ALT_SEL: c_int = 53;
pub const IMX6SLL_CLK_AXI_SEL: c_int = 54;
pub const IMX6SLL_CLK_PERIPH_PRE: c_int = 55;
pub const IMX6SLL_CLK_PERIPH2_PRE: c_int = 56;
pub const IMX6SLL_CLK_PERIPH_CLK2_SEL: c_int = 57;
pub const IMX6SLL_CLK_PERIPH2_CLK2_SEL: c_int = 58;
pub const IMX6SLL_CLK_PERCLK_SEL: c_int = 59;
pub const IMX6SLL_CLK_USDHC1_SEL: c_int = 60;
pub const IMX6SLL_CLK_USDHC2_SEL: c_int = 61;
pub const IMX6SLL_CLK_USDHC3_SEL: c_int = 62;
pub const IMX6SLL_CLK_SSI1_SEL: c_int = 63;
pub const IMX6SLL_CLK_SSI2_SEL: c_int = 64;
pub const IMX6SLL_CLK_SSI3_SEL: c_int = 65;
pub const IMX6SLL_CLK_PXP_SEL: c_int = 66;
pub const IMX6SLL_CLK_LCDIF_PRE_SEL: c_int = 67;
pub const IMX6SLL_CLK_LCDIF_SEL: c_int = 68;
pub const IMX6SLL_CLK_EPDC_PRE_SEL: c_int = 69;
pub const IMX6SLL_CLK_SPDIF_SEL: c_int = 70;
pub const IMX6SLL_CLK_ECSPI_SEL: c_int = 71;
pub const IMX6SLL_CLK_UART_SEL: c_int = 72;
pub const IMX6SLL_CLK_ARM: c_int = 73;
pub const IMX6SLL_CLK_PERIPH: c_int = 74;
pub const IMX6SLL_CLK_PERIPH2: c_int = 75;
pub const IMX6SLL_CLK_PERIPH2_CLK2: c_int = 76;
pub const IMX6SLL_CLK_PERIPH_CLK2: c_int = 77;
pub const IMX6SLL_CLK_MMDC_PODF: c_int = 78;
pub const IMX6SLL_CLK_AXI_PODF: c_int = 79;
pub const IMX6SLL_CLK_AHB: c_int = 80;
pub const IMX6SLL_CLK_IPG: c_int = 81;
pub const IMX6SLL_CLK_PERCLK: c_int = 82;
pub const IMX6SLL_CLK_USDHC1_PODF: c_int = 83;
pub const IMX6SLL_CLK_USDHC2_PODF: c_int = 84;
pub const IMX6SLL_CLK_USDHC3_PODF: c_int = 85;
pub const IMX6SLL_CLK_SSI1_PRED: c_int = 86;
pub const IMX6SLL_CLK_SSI2_PRED: c_int = 87;
pub const IMX6SLL_CLK_SSI3_PRED: c_int = 88;
pub const IMX6SLL_CLK_SSI1_PODF: c_int = 89;
pub const IMX6SLL_CLK_SSI2_PODF: c_int = 90;
pub const IMX6SLL_CLK_SSI3_PODF: c_int = 91;
pub const IMX6SLL_CLK_PXP_PODF: c_int = 92;
pub const IMX6SLL_CLK_LCDIF_PRED: c_int = 93;
pub const IMX6SLL_CLK_LCDIF_PODF: c_int = 94;
pub const IMX6SLL_CLK_EPDC_SEL: c_int = 95;
pub const IMX6SLL_CLK_EPDC_PODF: c_int = 96;
pub const IMX6SLL_CLK_SPDIF_PRED: c_int = 97;
pub const IMX6SLL_CLK_SPDIF_PODF: c_int = 98;
pub const IMX6SLL_CLK_ECSPI_PODF: c_int = 99;
pub const IMX6SLL_CLK_UART_PODF: c_int = 100;
// CCGR 0
pub const IMX6SLL_CLK_AIPSTZ1: c_int = 101;
pub const IMX6SLL_CLK_AIPSTZ2: c_int = 102;
pub const IMX6SLL_CLK_DCP: c_int = 103;
pub const IMX6SLL_CLK_UART2_IPG: c_int = 104;
pub const IMX6SLL_CLK_UART2_SERIAL: c_int = 105;
// CCGR 1
pub const IMX6SLL_CLK_ECSPI1: c_int = 106;
pub const IMX6SLL_CLK_ECSPI2: c_int = 107;
pub const IMX6SLL_CLK_ECSPI3: c_int = 108;
pub const IMX6SLL_CLK_ECSPI4: c_int = 109;
pub const IMX6SLL_CLK_UART3_IPG: c_int = 110;
pub const IMX6SLL_CLK_UART3_SERIAL: c_int = 111;
pub const IMX6SLL_CLK_UART4_IPG: c_int = 112;
pub const IMX6SLL_CLK_UART4_SERIAL: c_int = 113;
pub const IMX6SLL_CLK_EPIT1: c_int = 114;
pub const IMX6SLL_CLK_EPIT2: c_int = 115;
pub const IMX6SLL_CLK_GPT_BUS: c_int = 116;
pub const IMX6SLL_CLK_GPT_SERIAL: c_int = 117;
// CCGR2
pub const IMX6SLL_CLK_CSI: c_int = 118;
pub const IMX6SLL_CLK_I2C1: c_int = 119;
pub const IMX6SLL_CLK_I2C2: c_int = 120;
pub const IMX6SLL_CLK_I2C3: c_int = 121;
pub const IMX6SLL_CLK_OCOTP: c_int = 122;
pub const IMX6SLL_CLK_LCDIF_APB: c_int = 123;
pub const IMX6SLL_CLK_PXP: c_int = 124;
// CCGR3
pub const IMX6SLL_CLK_UART5_IPG: c_int = 125;
pub const IMX6SLL_CLK_UART5_SERIAL: c_int = 126;
pub const IMX6SLL_CLK_EPDC_AXI: c_int = 127;
pub const IMX6SLL_CLK_EPDC_PIX: c_int = 128;
pub const IMX6SLL_CLK_LCDIF_PIX: c_int = 129;
pub const IMX6SLL_CLK_WDOG1: c_int = 130;
pub const IMX6SLL_CLK_MMDC_P0_FAST: c_int = 131;
pub const IMX6SLL_CLK_MMDC_P0_IPG: c_int = 132;
pub const IMX6SLL_CLK_OCRAM: c_int = 133;
// CCGR4
pub const IMX6SLL_CLK_PWM1: c_int = 134;
pub const IMX6SLL_CLK_PWM2: c_int = 135;
pub const IMX6SLL_CLK_PWM3: c_int = 136;
pub const IMX6SLL_CLK_PWM4: c_int = 137;
// CCGR 5
pub const IMX6SLL_CLK_ROM: c_int = 138;
pub const IMX6SLL_CLK_SDMA: c_int = 139;
pub const IMX6SLL_CLK_KPP: c_int = 140;
pub const IMX6SLL_CLK_WDOG2: c_int = 141;
pub const IMX6SLL_CLK_SPBA: c_int = 142;
pub const IMX6SLL_CLK_SPDIF: c_int = 143;
pub const IMX6SLL_CLK_SPDIF_GCLK: c_int = 144;
pub const IMX6SLL_CLK_SSI1: c_int = 145;
pub const IMX6SLL_CLK_SSI1_IPG: c_int = 146;
pub const IMX6SLL_CLK_SSI2: c_int = 147;
pub const IMX6SLL_CLK_SSI2_IPG: c_int = 148;
pub const IMX6SLL_CLK_SSI3: c_int = 149;
pub const IMX6SLL_CLK_SSI3_IPG: c_int = 150;
pub const IMX6SLL_CLK_UART1_IPG: c_int = 151;
pub const IMX6SLL_CLK_UART1_SERIAL: c_int = 152;
// CCGR 6
pub const IMX6SLL_CLK_USBOH3: c_int = 153;
pub const IMX6SLL_CLK_USDHC1: c_int = 154;
pub const IMX6SLL_CLK_USDHC2: c_int = 155;
pub const IMX6SLL_CLK_USDHC3: c_int = 156;
pub const IMX6SLL_CLK_IPP_DI0: c_int = 157;
pub const IMX6SLL_CLK_IPP_DI1: c_int = 158;
pub const IMX6SLL_CLK_LDB_DI0_SEL: c_int = 159;
pub const IMX6SLL_CLK_LDB_DI0_DIV_3_5: c_int = 160;
pub const IMX6SLL_CLK_LDB_DI0_DIV_7: c_int = 161;
pub const IMX6SLL_CLK_LDB_DI0_DIV_SEL: c_int = 162;
pub const IMX6SLL_CLK_LDB_DI0: c_int = 163;
pub const IMX6SLL_CLK_LDB_DI1_SEL: c_int = 164;
pub const IMX6SLL_CLK_LDB_DI1_DIV_3_5: c_int = 165;
pub const IMX6SLL_CLK_LDB_DI1_DIV_7: c_int = 166;
pub const IMX6SLL_CLK_LDB_DI1_DIV_SEL: c_int = 167;
pub const IMX6SLL_CLK_LDB_DI1: c_int = 168;
pub const IMX6SLL_CLK_EXTERN_AUDIO_SEL: c_int = 169;
pub const IMX6SLL_CLK_EXTERN_AUDIO_PRED: c_int = 170;
pub const IMX6SLL_CLK_EXTERN_AUDIO_PODF: c_int = 171;
pub const IMX6SLL_CLK_EXTERN_AUDIO: c_int = 172;
pub const IMX6SLL_CLK_GPIO1: c_int = 173;
pub const IMX6SLL_CLK_GPIO2: c_int = 174;
pub const IMX6SLL_CLK_GPIO3: c_int = 175;
pub const IMX6SLL_CLK_GPIO4: c_int = 176;
pub const IMX6SLL_CLK_GPIO5: c_int = 177;
pub const IMX6SLL_CLK_GPIO6: c_int = 178;
pub const IMX6SLL_CLK_MMDC_P1_IPG: c_int = 179;
pub const IMX6SLL_CLK_END: c_int = 180;
