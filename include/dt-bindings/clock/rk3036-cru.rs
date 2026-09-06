//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/rk3036-cru.h
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
// Copyright (c) 2015 Rockchip Electronics Co. Ltd.
// Author: Xing Zheng <zhengxing@rock-chips.com>
//
// core clocks
pub const PLL_APLL: c_int = 1;
pub const PLL_DPLL: c_int = 2;
pub const PLL_GPLL: c_int = 3;
pub const ARMCLK: c_int = 4;
// sclk gates (special clocks)
pub const SCLK_GPU: c_int = 64;
pub const SCLK_SPI: c_int = 65;
pub const SCLK_SDMMC: c_int = 68;
pub const SCLK_SDIO: c_int = 69;
pub const SCLK_EMMC: c_int = 71;
pub const SCLK_NANDC: c_int = 76;
pub const SCLK_UART0: c_int = 77;
pub const SCLK_UART1: c_int = 78;
pub const SCLK_UART2: c_int = 79;
pub const SCLK_I2S: c_int = 82;
pub const SCLK_SPDIF: c_int = 83;
pub const SCLK_TIMER0: c_int = 85;
pub const SCLK_TIMER1: c_int = 86;
pub const SCLK_TIMER2: c_int = 87;
pub const SCLK_TIMER3: c_int = 88;
pub const SCLK_OTGPHY0: c_int = 93;
pub const SCLK_LCDC: c_int = 100;
pub const SCLK_HDMI: c_int = 109;
pub const SCLK_HEVC: c_int = 111;
pub const SCLK_I2S_OUT: c_int = 113;
pub const SCLK_SDMMC_DRV: c_int = 114;
pub const SCLK_SDIO_DRV: c_int = 115;
pub const SCLK_EMMC_DRV: c_int = 117;
pub const SCLK_SDMMC_SAMPLE: c_int = 118;
pub const SCLK_SDIO_SAMPLE: c_int = 119;
pub const SCLK_EMMC_SAMPLE: c_int = 121;
pub const SCLK_PVTM_CORE: c_int = 123;
pub const SCLK_PVTM_GPU: c_int = 124;
pub const SCLK_PVTM_VIDEO: c_int = 125;
pub const SCLK_MAC: c_int = 151;
pub const SCLK_MACREF: c_int = 152;
pub const SCLK_MACPLL: c_int = 153;
pub const SCLK_SFC: c_int = 160;
pub const SCLK_USB480M: c_int = 161;
// aclk gates
pub const ACLK_DMAC2: c_int = 194;
pub const ACLK_LCDC: c_int = 197;
pub const ACLK_VIO: c_int = 203;
pub const ACLK_VCODEC: c_int = 208;
pub const ACLK_CPU: c_int = 209;
pub const ACLK_PERI: c_int = 210;
// pclk gates
pub const PCLK_GPIO0: c_int = 320;
pub const PCLK_GPIO1: c_int = 321;
pub const PCLK_GPIO2: c_int = 322;
pub const PCLK_GRF: c_int = 329;
pub const PCLK_I2C0: c_int = 332;
pub const PCLK_I2C1: c_int = 333;
pub const PCLK_I2C2: c_int = 334;
pub const PCLK_SPI: c_int = 338;
pub const PCLK_UART0: c_int = 341;
pub const PCLK_UART1: c_int = 342;
pub const PCLK_UART2: c_int = 343;
pub const PCLK_PWM: c_int = 350;
pub const PCLK_TIMER: c_int = 353;
pub const PCLK_HDMI: c_int = 360;
pub const PCLK_CPU: c_int = 362;
pub const PCLK_PERI: c_int = 363;
pub const PCLK_DDRUPCTL: c_int = 364;
pub const PCLK_WDT: c_int = 368;
pub const PCLK_ACODEC: c_int = 369;
// hclk gates
pub const HCLK_OTG0: c_int = 449;
pub const HCLK_OTG1: c_int = 450;
pub const HCLK_NANDC: c_int = 453;
pub const HCLK_SFC: c_int = 454;
pub const HCLK_SDMMC: c_int = 456;
pub const HCLK_SDIO: c_int = 457;
pub const HCLK_EMMC: c_int = 459;
pub const HCLK_MAC: c_int = 460;
pub const HCLK_I2S: c_int = 462;
pub const HCLK_LCDC: c_int = 465;
pub const HCLK_ROM: c_int = 467;
pub const HCLK_VIO_BUS: c_int = 472;
pub const HCLK_VCODEC: c_int = 476;
pub const HCLK_CPU: c_int = 477;
pub const HCLK_PERI: c_int = 478;
// soft-reset indices
pub const SRST_CORE0: c_int = 0;
pub const SRST_CORE1: c_int = 1;
pub const SRST_CORE0_DBG: c_int = 4;
pub const SRST_CORE1_DBG: c_int = 5;
pub const SRST_CORE0_POR: c_int = 8;
pub const SRST_CORE1_POR: c_int = 9;
pub const SRST_L2C: c_int = 12;
pub const SRST_TOPDBG: c_int = 13;
pub const SRST_STRC_SYS_A: c_int = 14;
pub const SRST_PD_CORE_NIU: c_int = 15;
pub const SRST_TIMER2: c_int = 16;
pub const SRST_CPUSYS_H: c_int = 17;
pub const SRST_AHB2APB_H: c_int = 19;
pub const SRST_TIMER3: c_int = 20;
pub const SRST_INTMEM: c_int = 21;
pub const SRST_ROM: c_int = 22;
pub const SRST_PERI_NIU: c_int = 23;
pub const SRST_I2S: c_int = 24;
pub const SRST_DDR_PLL: c_int = 25;
pub const SRST_GPU_DLL: c_int = 26;
pub const SRST_TIMER0: c_int = 27;
pub const SRST_TIMER1: c_int = 28;
pub const SRST_CORE_DLL: c_int = 29;
pub const SRST_EFUSE_P: c_int = 30;
pub const SRST_ACODEC_P: c_int = 31;
pub const SRST_GPIO0: c_int = 32;
pub const SRST_GPIO1: c_int = 33;
pub const SRST_GPIO2: c_int = 34;
pub const SRST_UART0: c_int = 39;
pub const SRST_UART1: c_int = 40;
pub const SRST_UART2: c_int = 41;
pub const SRST_I2C0: c_int = 43;
pub const SRST_I2C1: c_int = 44;
pub const SRST_I2C2: c_int = 45;
pub const SRST_SFC: c_int = 47;
pub const SRST_PWM0: c_int = 48;
pub const SRST_DAP: c_int = 51;
pub const SRST_DAP_SYS: c_int = 52;
pub const SRST_GRF: c_int = 55;
pub const SRST_PERIPHSYS_A: c_int = 57;
pub const SRST_PERIPHSYS_H: c_int = 58;
pub const SRST_PERIPHSYS_P: c_int = 59;
pub const SRST_CPU_PERI: c_int = 61;
pub const SRST_EMEM_PERI: c_int = 62;
pub const SRST_USB_PERI: c_int = 63;
pub const SRST_DMA2: c_int = 64;
pub const SRST_MAC: c_int = 66;
pub const SRST_NANDC: c_int = 68;
pub const SRST_USBOTG0: c_int = 69;
pub const SRST_OTGC0: c_int = 71;
pub const SRST_USBOTG1: c_int = 72;
pub const SRST_OTGC1: c_int = 74;
pub const SRST_DDRMSCH: c_int = 79;
pub const SRST_MMC0: c_int = 81;
pub const SRST_SDIO: c_int = 82;
pub const SRST_EMMC: c_int = 83;
pub const SRST_SPI0: c_int = 84;
pub const SRST_WDT: c_int = 86;
pub const SRST_DDRPHY: c_int = 88;
pub const SRST_DDRPHY_P: c_int = 89;
pub const SRST_DDRCTRL: c_int = 90;
pub const SRST_DDRCTRL_P: c_int = 91;
pub const SRST_HDMI_P: c_int = 96;
pub const SRST_VIO_BUS_H: c_int = 99;
pub const SRST_UTMI0: c_int = 103;
pub const SRST_UTMI1: c_int = 104;
pub const SRST_USBPOR: c_int = 105;
pub const SRST_VCODEC_A: c_int = 112;
pub const SRST_VCODEC_H: c_int = 113;
pub const SRST_VIO1_A: c_int = 114;
pub const SRST_HEVC: c_int = 115;
pub const SRST_VCODEC_NIU_A: c_int = 116;
pub const SRST_LCDC1_A: c_int = 117;
pub const SRST_LCDC1_H: c_int = 118;
pub const SRST_LCDC1_D: c_int = 119;
pub const SRST_GPU: c_int = 120;
pub const SRST_GPU_NIU_A: c_int = 122;
pub const SRST_DBG_P: c_int = 131;
