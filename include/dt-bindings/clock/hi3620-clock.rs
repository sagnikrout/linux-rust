//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hi3620-clock.h
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
// Copyright (c) 2012-2013 Hisilicon Limited.
// Copyright (c) 2012-2013 Linaro Limited.
//
// Author: Haojian Zhuang <haojian.zhuang@linaro.org>
// Xin Li <li.xin@linaro.org>
//
pub const HI3620_NONE_CLOCK: c_int = 0;
// fixed rate & fixed factor clocks
pub const HI3620_OSC32K: c_int = 1;
pub const HI3620_OSC26M: c_int = 2;
pub const HI3620_PCLK: c_int = 3;
pub const HI3620_PLL_ARM0: c_int = 4;
pub const HI3620_PLL_ARM1: c_int = 5;
pub const HI3620_PLL_PERI: c_int = 6;
pub const HI3620_PLL_USB: c_int = 7;
pub const HI3620_PLL_HDMI: c_int = 8;
pub const HI3620_PLL_GPU: c_int = 9;
pub const HI3620_RCLK_TCXO: c_int = 10;
pub const HI3620_RCLK_CFGAXI: c_int = 11;
pub const HI3620_RCLK_PICO: c_int = 12;
// mux clocks
pub const HI3620_TIMER0_MUX: c_int = 32;
pub const HI3620_TIMER1_MUX: c_int = 33;
pub const HI3620_TIMER2_MUX: c_int = 34;
pub const HI3620_TIMER3_MUX: c_int = 35;
pub const HI3620_TIMER4_MUX: c_int = 36;
pub const HI3620_TIMER5_MUX: c_int = 37;
pub const HI3620_TIMER6_MUX: c_int = 38;
pub const HI3620_TIMER7_MUX: c_int = 39;
pub const HI3620_TIMER8_MUX: c_int = 40;
pub const HI3620_TIMER9_MUX: c_int = 41;
pub const HI3620_UART0_MUX: c_int = 42;
pub const HI3620_UART1_MUX: c_int = 43;
pub const HI3620_UART2_MUX: c_int = 44;
pub const HI3620_UART3_MUX: c_int = 45;
pub const HI3620_UART4_MUX: c_int = 46;
pub const HI3620_SPI0_MUX: c_int = 47;
pub const HI3620_SPI1_MUX: c_int = 48;
pub const HI3620_SPI2_MUX: c_int = 49;
pub const HI3620_SAXI_MUX: c_int = 50;
pub const HI3620_PWM0_MUX: c_int = 51;
pub const HI3620_PWM1_MUX: c_int = 52;
pub const HI3620_SD_MUX: c_int = 53;
pub const HI3620_MMC1_MUX: c_int = 54;
pub const HI3620_MMC1_MUX2: c_int = 55;
pub const HI3620_G2D_MUX: c_int = 56;
pub const HI3620_VENC_MUX: c_int = 57;
pub const HI3620_VDEC_MUX: c_int = 58;
pub const HI3620_VPP_MUX: c_int = 59;
pub const HI3620_EDC0_MUX: c_int = 60;
pub const HI3620_LDI0_MUX: c_int = 61;
pub const HI3620_EDC1_MUX: c_int = 62;
pub const HI3620_LDI1_MUX: c_int = 63;
pub const HI3620_RCLK_HSIC: c_int = 64;
pub const HI3620_MMC2_MUX: c_int = 65;
pub const HI3620_MMC3_MUX: c_int = 66;
// divider clocks
pub const HI3620_SHAREAXI_DIV: c_int = 128;
pub const HI3620_CFGAXI_DIV: c_int = 129;
pub const HI3620_SD_DIV: c_int = 130;
pub const HI3620_MMC1_DIV: c_int = 131;
pub const HI3620_HSIC_DIV: c_int = 132;
pub const HI3620_MMC2_DIV: c_int = 133;
pub const HI3620_MMC3_DIV: c_int = 134;
// gate clocks
pub const HI3620_TIMERCLK01: c_int = 160;
pub const HI3620_TIMER_RCLK01: c_int = 161;
pub const HI3620_TIMERCLK23: c_int = 162;
pub const HI3620_TIMER_RCLK23: c_int = 163;
pub const HI3620_TIMERCLK45: c_int = 164;
pub const HI3620_TIMERCLK67: c_int = 165;
pub const HI3620_TIMERCLK89: c_int = 166;
pub const HI3620_RTCCLK: c_int = 167;
pub const HI3620_KPC_CLK: c_int = 168;
pub const HI3620_GPIOCLK0: c_int = 169;
pub const HI3620_GPIOCLK1: c_int = 170;
pub const HI3620_GPIOCLK2: c_int = 171;
pub const HI3620_GPIOCLK3: c_int = 172;
pub const HI3620_GPIOCLK4: c_int = 173;
pub const HI3620_GPIOCLK5: c_int = 174;
pub const HI3620_GPIOCLK6: c_int = 175;
pub const HI3620_GPIOCLK7: c_int = 176;
pub const HI3620_GPIOCLK8: c_int = 177;
pub const HI3620_GPIOCLK9: c_int = 178;
pub const HI3620_GPIOCLK10: c_int = 179;
pub const HI3620_GPIOCLK11: c_int = 180;
pub const HI3620_GPIOCLK12: c_int = 181;
pub const HI3620_GPIOCLK13: c_int = 182;
pub const HI3620_GPIOCLK14: c_int = 183;
pub const HI3620_GPIOCLK15: c_int = 184;
pub const HI3620_GPIOCLK16: c_int = 185;
pub const HI3620_GPIOCLK17: c_int = 186;
pub const HI3620_GPIOCLK18: c_int = 187;
pub const HI3620_GPIOCLK19: c_int = 188;
pub const HI3620_GPIOCLK20: c_int = 189;
pub const HI3620_GPIOCLK21: c_int = 190;
pub const HI3620_DPHY0_CLK: c_int = 191;
pub const HI3620_DPHY1_CLK: c_int = 192;
pub const HI3620_DPHY2_CLK: c_int = 193;
pub const HI3620_USBPHY_CLK: c_int = 194;
pub const HI3620_ACP_CLK: c_int = 195;
pub const HI3620_PWMCLK0: c_int = 196;
pub const HI3620_PWMCLK1: c_int = 197;
pub const HI3620_UARTCLK0: c_int = 198;
pub const HI3620_UARTCLK1: c_int = 199;
pub const HI3620_UARTCLK2: c_int = 200;
pub const HI3620_UARTCLK3: c_int = 201;
pub const HI3620_UARTCLK4: c_int = 202;
pub const HI3620_SPICLK0: c_int = 203;
pub const HI3620_SPICLK1: c_int = 204;
pub const HI3620_SPICLK2: c_int = 205;
pub const HI3620_I2CCLK0: c_int = 206;
pub const HI3620_I2CCLK1: c_int = 207;
pub const HI3620_I2CCLK2: c_int = 208;
pub const HI3620_I2CCLK3: c_int = 209;
pub const HI3620_SCI_CLK: c_int = 210;
pub const HI3620_DDRC_PER_CLK: c_int = 211;
pub const HI3620_DMAC_CLK: c_int = 212;
pub const HI3620_USB2DVC_CLK: c_int = 213;
pub const HI3620_SD_CLK: c_int = 214;
pub const HI3620_MMC_CLK1: c_int = 215;
pub const HI3620_MMC_CLK2: c_int = 216;
pub const HI3620_MMC_CLK3: c_int = 217;
pub const HI3620_MCU_CLK: c_int = 218;
pub const HI3620_SD_CIUCLK: c_int = 0;
pub const HI3620_MMC_CIUCLK1: c_int = 1;
pub const HI3620_MMC_CIUCLK2: c_int = 2;
pub const HI3620_MMC_CIUCLK3: c_int = 3;
pub const HI3620_NR_CLKS: c_int = 219;
