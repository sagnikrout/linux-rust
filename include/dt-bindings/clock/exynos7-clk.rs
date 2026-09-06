//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/exynos7-clk.h
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Naveen Krishna Ch <naveenkrishna.ch@gmail.com>
//
// TOPC
pub const DOUT_ACLK_PERIS: c_int = 1;
pub const DOUT_SCLK_BUS0_PLL: c_int = 2;
pub const DOUT_SCLK_BUS1_PLL: c_int = 3;
pub const DOUT_SCLK_CC_PLL: c_int = 4;
pub const DOUT_SCLK_MFC_PLL: c_int = 5;
pub const DOUT_ACLK_CCORE_133: c_int = 6;
pub const DOUT_ACLK_MSCL_532: c_int = 7;
pub const ACLK_MSCL_532: c_int = 8;
pub const DOUT_SCLK_AUD_PLL: c_int = 9;
pub const FOUT_AUD_PLL: c_int = 10;
pub const SCLK_AUD_PLL: c_int = 11;
pub const SCLK_MFC_PLL_B: c_int = 12;
pub const SCLK_MFC_PLL_A: c_int = 13;
pub const SCLK_BUS1_PLL_B: c_int = 14;
pub const SCLK_BUS1_PLL_A: c_int = 15;
pub const SCLK_BUS0_PLL_B: c_int = 16;
pub const SCLK_BUS0_PLL_A: c_int = 17;
pub const SCLK_CC_PLL_B: c_int = 18;
pub const SCLK_CC_PLL_A: c_int = 19;
pub const ACLK_CCORE_133: c_int = 20;
pub const ACLK_PERIS_66: c_int = 21;
pub const TOPC_NR_CLK: c_int = 22;
// TOP0
pub const DOUT_ACLK_PERIC1: c_int = 1;
pub const DOUT_ACLK_PERIC0: c_int = 2;
pub const CLK_SCLK_UART0: c_int = 3;
pub const CLK_SCLK_UART1: c_int = 4;
pub const CLK_SCLK_UART2: c_int = 5;
pub const CLK_SCLK_UART3: c_int = 6;
pub const CLK_SCLK_SPI0: c_int = 7;
pub const CLK_SCLK_SPI1: c_int = 8;
pub const CLK_SCLK_SPI2: c_int = 9;
pub const CLK_SCLK_SPI3: c_int = 10;
pub const CLK_SCLK_SPI4: c_int = 11;
pub const CLK_SCLK_SPDIF: c_int = 12;
pub const CLK_SCLK_PCM1: c_int = 13;
pub const CLK_SCLK_I2S1: c_int = 14;
pub const CLK_ACLK_PERIC0_66: c_int = 15;
pub const CLK_ACLK_PERIC1_66: c_int = 16;
pub const TOP0_NR_CLK: c_int = 17;
// TOP1
pub const DOUT_ACLK_FSYS1_200: c_int = 1;
pub const DOUT_ACLK_FSYS0_200: c_int = 2;
pub const DOUT_SCLK_MMC2: c_int = 3;
pub const DOUT_SCLK_MMC1: c_int = 4;
pub const DOUT_SCLK_MMC0: c_int = 5;
pub const CLK_SCLK_MMC2: c_int = 6;
pub const CLK_SCLK_MMC1: c_int = 7;
pub const CLK_SCLK_MMC0: c_int = 8;
pub const CLK_ACLK_FSYS0_200: c_int = 9;
pub const CLK_ACLK_FSYS1_200: c_int = 10;
pub const CLK_SCLK_PHY_FSYS1: c_int = 11;
pub const CLK_SCLK_PHY_FSYS1_26M: c_int = 12;
pub const MOUT_SCLK_UFSUNIPRO20: c_int = 13;
pub const DOUT_SCLK_UFSUNIPRO20: c_int = 14;
pub const CLK_SCLK_UFSUNIPRO20: c_int = 15;
pub const DOUT_SCLK_PHY_FSYS1: c_int = 16;
pub const DOUT_SCLK_PHY_FSYS1_26M: c_int = 17;
pub const TOP1_NR_CLK: c_int = 18;
// CCORE
pub const PCLK_RTC: c_int = 1;
pub const CCORE_NR_CLK: c_int = 2;
// PERIC0
pub const PCLK_UART0: c_int = 1;
pub const SCLK_UART0: c_int = 2;
pub const PCLK_HSI2C0: c_int = 3;
pub const PCLK_HSI2C1: c_int = 4;
pub const PCLK_HSI2C4: c_int = 5;
pub const PCLK_HSI2C5: c_int = 6;
pub const PCLK_HSI2C9: c_int = 7;
pub const PCLK_HSI2C10: c_int = 8;
pub const PCLK_HSI2C11: c_int = 9;
pub const PCLK_PWM: c_int = 10;
pub const SCLK_PWM: c_int = 11;
pub const PCLK_ADCIF: c_int = 12;
pub const PERIC0_NR_CLK: c_int = 13;
// PERIC1
pub const PCLK_UART1: c_int = 1;
pub const PCLK_UART2: c_int = 2;
pub const PCLK_UART3: c_int = 3;
pub const SCLK_UART1: c_int = 4;
pub const SCLK_UART2: c_int = 5;
pub const SCLK_UART3: c_int = 6;
pub const PCLK_HSI2C2: c_int = 7;
pub const PCLK_HSI2C3: c_int = 8;
pub const PCLK_HSI2C6: c_int = 9;
pub const PCLK_HSI2C7: c_int = 10;
pub const PCLK_HSI2C8: c_int = 11;
pub const PCLK_SPI0: c_int = 12;
pub const PCLK_SPI1: c_int = 13;
pub const PCLK_SPI2: c_int = 14;
pub const PCLK_SPI3: c_int = 15;
pub const PCLK_SPI4: c_int = 16;
pub const SCLK_SPI0: c_int = 17;
pub const SCLK_SPI1: c_int = 18;
pub const SCLK_SPI2: c_int = 19;
pub const SCLK_SPI3: c_int = 20;
pub const SCLK_SPI4: c_int = 21;
pub const PCLK_I2S1: c_int = 22;
pub const PCLK_PCM1: c_int = 23;
pub const PCLK_SPDIF: c_int = 24;
pub const SCLK_I2S1: c_int = 25;
pub const SCLK_PCM1: c_int = 26;
pub const SCLK_SPDIF: c_int = 27;
pub const PERIC1_NR_CLK: c_int = 28;
// PERIS
pub const PCLK_CHIPID: c_int = 1;
pub const SCLK_CHIPID: c_int = 2;
pub const PCLK_WDT: c_int = 3;
pub const PCLK_TMU: c_int = 4;
pub const SCLK_TMU: c_int = 5;
pub const PERIS_NR_CLK: c_int = 6;
// FSYS0
pub const ACLK_MMC2: c_int = 1;
pub const ACLK_AXIUS_USBDRD30X_FSYS0X: c_int = 2;
pub const ACLK_USBDRD300: c_int = 3;
pub const SCLK_USBDRD300_SUSPENDCLK: c_int = 4;
pub const SCLK_USBDRD300_REFCLK: c_int = 5;
pub const PHYCLK_USBDRD300_UDRD30_PIPE_PCLK_USER: c_int = 6;
pub const PHYCLK_USBDRD300_UDRD30_PHYCLK_USER: c_int = 7;
pub const OSCCLK_PHY_CLKOUT_USB30_PHY: c_int = 8;
pub const ACLK_PDMA0: c_int = 9;
pub const ACLK_PDMA1: c_int = 10;
pub const FSYS0_NR_CLK: c_int = 11;
// FSYS1
pub const ACLK_MMC1: c_int = 1;
pub const ACLK_MMC0: c_int = 2;
pub const PHYCLK_UFS20_TX0_SYMBOL: c_int = 3;
pub const PHYCLK_UFS20_RX0_SYMBOL: c_int = 4;
pub const PHYCLK_UFS20_RX1_SYMBOL: c_int = 5;
pub const ACLK_UFS20_LINK: c_int = 6;
pub const SCLK_UFSUNIPRO20_USER: c_int = 7;
pub const PHYCLK_UFS20_RX1_SYMBOL_USER: c_int = 8;
pub const PHYCLK_UFS20_RX0_SYMBOL_USER: c_int = 9;
pub const PHYCLK_UFS20_TX0_SYMBOL_USER: c_int = 10;
pub const OSCCLK_PHY_CLKOUT_EMBEDDED_COMBO_PHY: c_int = 11;
pub const SCLK_COMBO_PHY_EMBEDDED_26M: c_int = 12;
pub const DOUT_PCLK_FSYS1: c_int = 13;
pub const PCLK_GPIO_FSYS1: c_int = 14;
pub const MOUT_FSYS1_PHYCLK_SEL1: c_int = 15;
pub const FSYS1_NR_CLK: c_int = 16;
// MSCL
pub const USERMUX_ACLK_MSCL_532: c_int = 1;
pub const DOUT_PCLK_MSCL: c_int = 2;
pub const ACLK_MSCL_0: c_int = 3;
pub const ACLK_MSCL_1: c_int = 4;
pub const ACLK_JPEG: c_int = 5;
pub const ACLK_G2D: c_int = 6;
pub const ACLK_LH_ASYNC_SI_MSCL_0: c_int = 7;
pub const ACLK_LH_ASYNC_SI_MSCL_1: c_int = 8;
pub const ACLK_AXI2ACEL_BRIDGE: c_int = 9;
pub const ACLK_XIU_MSCLX_0: c_int = 10;
pub const ACLK_XIU_MSCLX_1: c_int = 11;
pub const ACLK_QE_MSCL_0: c_int = 12;
pub const ACLK_QE_MSCL_1: c_int = 13;
pub const ACLK_QE_JPEG: c_int = 14;
pub const ACLK_QE_G2D: c_int = 15;
pub const ACLK_PPMU_MSCL_0: c_int = 16;
pub const ACLK_PPMU_MSCL_1: c_int = 17;
pub const ACLK_MSCLNP_133: c_int = 18;
pub const ACLK_AHB2APB_MSCL0P: c_int = 19;
pub const ACLK_AHB2APB_MSCL1P: c_int = 20;
pub const PCLK_MSCL_0: c_int = 21;
pub const PCLK_MSCL_1: c_int = 22;
pub const PCLK_JPEG: c_int = 23;
pub const PCLK_G2D: c_int = 24;
pub const PCLK_QE_MSCL_0: c_int = 25;
pub const PCLK_QE_MSCL_1: c_int = 26;
pub const PCLK_QE_JPEG: c_int = 27;
pub const PCLK_QE_G2D: c_int = 28;
pub const PCLK_PPMU_MSCL_0: c_int = 29;
pub const PCLK_PPMU_MSCL_1: c_int = 30;
pub const PCLK_AXI2ACEL_BRIDGE: c_int = 31;
pub const PCLK_PMU_MSCL: c_int = 32;
pub const MSCL_NR_CLK: c_int = 33;
// AUD
pub const SCLK_I2S: c_int = 1;
pub const SCLK_PCM: c_int = 2;
pub const PCLK_I2S: c_int = 3;
pub const PCLK_PCM: c_int = 4;
pub const ACLK_ADMA: c_int = 5;
pub const AUD_NR_CLK: c_int = 6;
