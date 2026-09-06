//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hi6220-clock.h
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
// Copyright (c) 2015 Hisilicon Limited.
//
// Author: Bintian Wang <bintian.wang@huawei.com>
//
// clk in Hi6220 AO (always on) controller
pub const HI6220_NONE_CLOCK: c_int = 0;
// fixed rate clocks
pub const HI6220_REF32K: c_int = 1;
pub const HI6220_CLK_TCXO: c_int = 2;
pub const HI6220_MMC1_PAD: c_int = 3;
pub const HI6220_MMC2_PAD: c_int = 4;
pub const HI6220_MMC0_PAD: c_int = 5;
pub const HI6220_PLL_BBP: c_int = 6;
pub const HI6220_PLL_GPU: c_int = 7;
pub const HI6220_PLL1_DDR: c_int = 8;
pub const HI6220_PLL_SYS: c_int = 9;
pub const HI6220_PLL_SYS_MEDIA: c_int = 10;
pub const HI6220_DDR_SRC: c_int = 11;
pub const HI6220_PLL_MEDIA: c_int = 12;
pub const HI6220_PLL_DDR: c_int = 13;
// fixed factor clocks
pub const HI6220_300M: c_int = 14;
pub const HI6220_150M: c_int = 15;
pub const HI6220_PICOPHY_SRC: c_int = 16;
pub const HI6220_MMC0_SRC_SEL: c_int = 17;
pub const HI6220_MMC1_SRC_SEL: c_int = 18;
pub const HI6220_MMC2_SRC_SEL: c_int = 19;
pub const HI6220_VPU_CODEC: c_int = 20;
pub const HI6220_MMC0_SMP: c_int = 21;
pub const HI6220_MMC1_SMP: c_int = 22;
pub const HI6220_MMC2_SMP: c_int = 23;
// gate clocks
pub const HI6220_WDT0_PCLK: c_int = 24;
pub const HI6220_WDT1_PCLK: c_int = 25;
pub const HI6220_WDT2_PCLK: c_int = 26;
pub const HI6220_TIMER0_PCLK: c_int = 27;
pub const HI6220_TIMER1_PCLK: c_int = 28;
pub const HI6220_TIMER2_PCLK: c_int = 29;
pub const HI6220_TIMER3_PCLK: c_int = 30;
pub const HI6220_TIMER4_PCLK: c_int = 31;
pub const HI6220_TIMER5_PCLK: c_int = 32;
pub const HI6220_TIMER6_PCLK: c_int = 33;
pub const HI6220_TIMER7_PCLK: c_int = 34;
pub const HI6220_TIMER8_PCLK: c_int = 35;
pub const HI6220_UART0_PCLK: c_int = 36;
pub const HI6220_RTC0_PCLK: c_int = 37;
pub const HI6220_RTC1_PCLK: c_int = 38;
pub const HI6220_AO_NR_CLKS: c_int = 39;
// clk in Hi6220 systrl
// gate clock
pub const HI6220_MMC0_CLK: c_int = 1;
pub const HI6220_MMC0_CIUCLK: c_int = 2;
pub const HI6220_MMC1_CLK: c_int = 3;
pub const HI6220_MMC1_CIUCLK: c_int = 4;
pub const HI6220_MMC2_CLK: c_int = 5;
pub const HI6220_MMC2_CIUCLK: c_int = 6;
pub const HI6220_USBOTG_HCLK: c_int = 7;
pub const HI6220_CLK_PICOPHY: c_int = 8;
pub const HI6220_HIFI: c_int = 9;
pub const HI6220_DACODEC_PCLK: c_int = 10;
pub const HI6220_EDMAC_ACLK: c_int = 11;
pub const HI6220_CS_ATB: c_int = 12;
pub const HI6220_I2C0_CLK: c_int = 13;
pub const HI6220_I2C1_CLK: c_int = 14;
pub const HI6220_I2C2_CLK: c_int = 15;
pub const HI6220_I2C3_CLK: c_int = 16;
pub const HI6220_UART1_PCLK: c_int = 17;
pub const HI6220_UART2_PCLK: c_int = 18;
pub const HI6220_UART3_PCLK: c_int = 19;
pub const HI6220_UART4_PCLK: c_int = 20;
pub const HI6220_SPI_CLK: c_int = 21;
pub const HI6220_TSENSOR_CLK: c_int = 22;
pub const HI6220_MMU_CLK: c_int = 23;
pub const HI6220_HIFI_SEL: c_int = 24;
pub const HI6220_MMC0_SYSPLL: c_int = 25;
pub const HI6220_MMC1_SYSPLL: c_int = 26;
pub const HI6220_MMC2_SYSPLL: c_int = 27;
pub const HI6220_MMC0_SEL: c_int = 28;
pub const HI6220_MMC1_SEL: c_int = 29;
pub const HI6220_BBPPLL_SEL: c_int = 30;
pub const HI6220_MEDIA_PLL_SRC: c_int = 31;
pub const HI6220_MMC2_SEL: c_int = 32;
pub const HI6220_CS_ATB_SYSPLL: c_int = 33;
// mux clocks
pub const HI6220_MMC0_SRC: c_int = 34;
pub const HI6220_MMC0_SMP_IN: c_int = 35;
pub const HI6220_MMC1_SRC: c_int = 36;
pub const HI6220_MMC1_SMP_IN: c_int = 37;
pub const HI6220_MMC2_SRC: c_int = 38;
pub const HI6220_MMC2_SMP_IN: c_int = 39;
pub const HI6220_HIFI_SRC: c_int = 40;
pub const HI6220_UART1_SRC: c_int = 41;
pub const HI6220_UART2_SRC: c_int = 42;
pub const HI6220_UART3_SRC: c_int = 43;
pub const HI6220_UART4_SRC: c_int = 44;
pub const HI6220_MMC0_MUX0: c_int = 45;
pub const HI6220_MMC1_MUX0: c_int = 46;
pub const HI6220_MMC2_MUX0: c_int = 47;
pub const HI6220_MMC0_MUX1: c_int = 48;
pub const HI6220_MMC1_MUX1: c_int = 49;
pub const HI6220_MMC2_MUX1: c_int = 50;
// divider clocks
pub const HI6220_CLK_BUS: c_int = 51;
pub const HI6220_MMC0_DIV: c_int = 52;
pub const HI6220_MMC1_DIV: c_int = 53;
pub const HI6220_MMC2_DIV: c_int = 54;
pub const HI6220_HIFI_DIV: c_int = 55;
pub const HI6220_BBPPLL0_DIV: c_int = 56;
pub const HI6220_CS_DAPB: c_int = 57;
pub const HI6220_CS_ATB_DIV: c_int = 58;
// gate clock
pub const HI6220_DAPB_CLK: c_int = 59;
pub const HI6220_SYS_NR_CLKS: c_int = 60;
// clk in Hi6220 media controller
// gate clocks
pub const HI6220_DSI_PCLK: c_int = 1;
pub const HI6220_G3D_PCLK: c_int = 2;
pub const HI6220_ACLK_CODEC_VPU: c_int = 3;
pub const HI6220_ISP_SCLK: c_int = 4;
pub const HI6220_ADE_CORE: c_int = 5;
pub const HI6220_MED_MMU: c_int = 6;
pub const HI6220_CFG_CSI4PHY: c_int = 7;
pub const HI6220_CFG_CSI2PHY: c_int = 8;
pub const HI6220_ISP_SCLK_GATE: c_int = 9;
pub const HI6220_ISP_SCLK_GATE1: c_int = 10;
pub const HI6220_ADE_CORE_GATE: c_int = 11;
pub const HI6220_CODEC_VPU_GATE: c_int = 12;
pub const HI6220_MED_SYSPLL: c_int = 13;
// mux clocks
pub const HI6220_1440_1200: c_int = 14;
pub const HI6220_1000_1200: c_int = 15;
pub const HI6220_1000_1440: c_int = 16;
// divider clocks
pub const HI6220_CODEC_JPEG: c_int = 17;
pub const HI6220_ISP_SCLK_SRC: c_int = 18;
pub const HI6220_ISP_SCLK1: c_int = 19;
pub const HI6220_ADE_CORE_SRC: c_int = 20;
pub const HI6220_ADE_PIX_SRC: c_int = 21;
pub const HI6220_G3D_CLK: c_int = 22;
pub const HI6220_CODEC_VPU_SRC: c_int = 23;
pub const HI6220_MEDIA_NR_CLKS: c_int = 24;
// clk in Hi6220 power controller
// gate clocks
pub const HI6220_PLL_GPU_GATE: c_int = 1;
pub const HI6220_PLL1_DDR_GATE: c_int = 2;
pub const HI6220_PLL_DDR_GATE: c_int = 3;
pub const HI6220_PLL_MEDIA_GATE: c_int = 4;
pub const HI6220_PLL0_BBP_GATE: c_int = 5;
// divider clocks
pub const HI6220_DDRC_SRC: c_int = 6;
pub const HI6220_DDRC_AXI1: c_int = 7;
pub const HI6220_POWER_NR_CLKS: c_int = 8;
// clk in Hi6220 acpu sctrl
pub const HI6220_ACPU_SFT_AT_S: c_int = 0;
