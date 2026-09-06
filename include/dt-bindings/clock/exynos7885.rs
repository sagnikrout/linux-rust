//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/exynos7885.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2021 Dávid Virág
//
// Device Tree binding constants for Exynos7885 clock controller.
//
// CMU_TOP
pub const CLK_FOUT_SHARED0_PLL: c_int = 1;
pub const CLK_FOUT_SHARED1_PLL: c_int = 2;
pub const CLK_DOUT_SHARED0_DIV2: c_int = 3;
pub const CLK_DOUT_SHARED0_DIV3: c_int = 4;
pub const CLK_DOUT_SHARED0_DIV4: c_int = 5;
pub const CLK_DOUT_SHARED0_DIV5: c_int = 6;
pub const CLK_DOUT_SHARED1_DIV2: c_int = 7;
pub const CLK_DOUT_SHARED1_DIV3: c_int = 8;
pub const CLK_DOUT_SHARED1_DIV4: c_int = 9;
pub const CLK_MOUT_CORE_BUS: c_int = 10;
pub const CLK_MOUT_CORE_CCI: c_int = 11;
pub const CLK_MOUT_CORE_G3D: c_int = 12;
pub const CLK_DOUT_CORE_BUS: c_int = 13;
pub const CLK_DOUT_CORE_CCI: c_int = 14;
pub const CLK_DOUT_CORE_G3D: c_int = 15;
pub const CLK_GOUT_CORE_BUS: c_int = 16;
pub const CLK_GOUT_CORE_CCI: c_int = 17;
pub const CLK_GOUT_CORE_G3D: c_int = 18;
pub const CLK_MOUT_PERI_BUS: c_int = 19;
pub const CLK_MOUT_PERI_SPI0: c_int = 20;
pub const CLK_MOUT_PERI_SPI1: c_int = 21;
pub const CLK_MOUT_PERI_UART0: c_int = 22;
pub const CLK_MOUT_PERI_UART1: c_int = 23;
pub const CLK_MOUT_PERI_UART2: c_int = 24;
pub const CLK_MOUT_PERI_USI0: c_int = 25;
pub const CLK_MOUT_PERI_USI1: c_int = 26;
pub const CLK_MOUT_PERI_USI2: c_int = 27;
pub const CLK_DOUT_PERI_BUS: c_int = 28;
pub const CLK_DOUT_PERI_SPI0: c_int = 29;
pub const CLK_DOUT_PERI_SPI1: c_int = 30;
pub const CLK_DOUT_PERI_UART0: c_int = 31;
pub const CLK_DOUT_PERI_UART1: c_int = 32;
pub const CLK_DOUT_PERI_UART2: c_int = 33;
pub const CLK_DOUT_PERI_USI0: c_int = 34;
pub const CLK_DOUT_PERI_USI1: c_int = 35;
pub const CLK_DOUT_PERI_USI2: c_int = 36;
pub const CLK_GOUT_PERI_BUS: c_int = 37;
pub const CLK_GOUT_PERI_SPI0: c_int = 38;
pub const CLK_GOUT_PERI_SPI1: c_int = 39;
pub const CLK_GOUT_PERI_UART0: c_int = 40;
pub const CLK_GOUT_PERI_UART1: c_int = 41;
pub const CLK_GOUT_PERI_UART2: c_int = 42;
pub const CLK_GOUT_PERI_USI0: c_int = 43;
pub const CLK_GOUT_PERI_USI1: c_int = 44;
pub const CLK_GOUT_PERI_USI2: c_int = 45;
pub const CLK_MOUT_FSYS_BUS: c_int = 46;
pub const CLK_MOUT_FSYS_MMC_CARD: c_int = 47;
pub const CLK_MOUT_FSYS_MMC_EMBD: c_int = 48;
pub const CLK_MOUT_FSYS_MMC_SDIO: c_int = 49;
pub const CLK_MOUT_FSYS_USB30DRD: c_int = 50;
pub const CLK_DOUT_FSYS_BUS: c_int = 51;
pub const CLK_DOUT_FSYS_MMC_CARD: c_int = 52;
pub const CLK_DOUT_FSYS_MMC_EMBD: c_int = 53;
pub const CLK_DOUT_FSYS_MMC_SDIO: c_int = 54;
pub const CLK_DOUT_FSYS_USB30DRD: c_int = 55;
pub const CLK_GOUT_FSYS_BUS: c_int = 56;
pub const CLK_GOUT_FSYS_MMC_CARD: c_int = 57;
pub const CLK_GOUT_FSYS_MMC_EMBD: c_int = 58;
pub const CLK_GOUT_FSYS_MMC_SDIO: c_int = 59;
pub const CLK_GOUT_FSYS_USB30DRD: c_int = 60;
pub const CLK_MOUT_SHARED0_PLL: c_int = 61;
pub const CLK_MOUT_SHARED1_PLL: c_int = 62;
// CMU_CORE
pub const CLK_MOUT_CORE_BUS_USER: c_int = 1;
pub const CLK_MOUT_CORE_CCI_USER: c_int = 2;
pub const CLK_MOUT_CORE_G3D_USER: c_int = 3;
pub const CLK_MOUT_CORE_GIC: c_int = 4;
pub const CLK_DOUT_CORE_BUSP: c_int = 5;
pub const CLK_GOUT_CCI_ACLK: c_int = 6;
pub const CLK_GOUT_GIC400_CLK: c_int = 7;
pub const CLK_GOUT_TREX_D_CORE_ACLK: c_int = 8;
pub const CLK_GOUT_TREX_D_CORE_GCLK: c_int = 9;
pub const CLK_GOUT_TREX_D_CORE_PCLK: c_int = 10;
pub const CLK_GOUT_TREX_P_CORE_ACLK_P_CORE: c_int = 11;
pub const CLK_GOUT_TREX_P_CORE_CCLK_P_CORE: c_int = 12;
pub const CLK_GOUT_TREX_P_CORE_PCLK: c_int = 13;
pub const CLK_GOUT_TREX_P_CORE_PCLK_P_CORE: c_int = 14;
// CMU_PERI
pub const CLK_MOUT_PERI_BUS_USER: c_int = 1;
pub const CLK_MOUT_PERI_SPI0_USER: c_int = 2;
pub const CLK_MOUT_PERI_SPI1_USER: c_int = 3;
pub const CLK_MOUT_PERI_UART0_USER: c_int = 4;
pub const CLK_MOUT_PERI_UART1_USER: c_int = 5;
pub const CLK_MOUT_PERI_UART2_USER: c_int = 6;
pub const CLK_MOUT_PERI_USI0_USER: c_int = 7;
pub const CLK_MOUT_PERI_USI1_USER: c_int = 8;
pub const CLK_MOUT_PERI_USI2_USER: c_int = 9;
pub const CLK_GOUT_GPIO_TOP_PCLK: c_int = 10;
pub const CLK_GOUT_HSI2C0_PCLK: c_int = 11;
pub const CLK_GOUT_HSI2C1_PCLK: c_int = 12;
pub const CLK_GOUT_HSI2C2_PCLK: c_int = 13;
pub const CLK_GOUT_HSI2C3_PCLK: c_int = 14;
pub const CLK_GOUT_I2C0_PCLK: c_int = 15;
pub const CLK_GOUT_I2C1_PCLK: c_int = 16;
pub const CLK_GOUT_I2C2_PCLK: c_int = 17;
pub const CLK_GOUT_I2C3_PCLK: c_int = 18;
pub const CLK_GOUT_I2C4_PCLK: c_int = 19;
pub const CLK_GOUT_I2C5_PCLK: c_int = 20;
pub const CLK_GOUT_I2C6_PCLK: c_int = 21;
pub const CLK_GOUT_I2C7_PCLK: c_int = 22;
pub const CLK_GOUT_PWM_MOTOR_PCLK: c_int = 23;
pub const CLK_GOUT_SPI0_PCLK: c_int = 24;
pub const CLK_GOUT_SPI0_EXT_CLK: c_int = 25;
pub const CLK_GOUT_SPI1_PCLK: c_int = 26;
pub const CLK_GOUT_SPI1_EXT_CLK: c_int = 27;
pub const CLK_GOUT_UART0_EXT_UCLK: c_int = 28;
pub const CLK_GOUT_UART0_PCLK: c_int = 29;
pub const CLK_GOUT_UART1_EXT_UCLK: c_int = 30;
pub const CLK_GOUT_UART1_PCLK: c_int = 31;
pub const CLK_GOUT_UART2_EXT_UCLK: c_int = 32;
pub const CLK_GOUT_UART2_PCLK: c_int = 33;
pub const CLK_GOUT_USI0_PCLK: c_int = 34;
pub const CLK_GOUT_USI0_SCLK: c_int = 35;
pub const CLK_GOUT_USI1_PCLK: c_int = 36;
pub const CLK_GOUT_USI1_SCLK: c_int = 37;
pub const CLK_GOUT_USI2_PCLK: c_int = 38;
pub const CLK_GOUT_USI2_SCLK: c_int = 39;
pub const CLK_GOUT_MCT_PCLK: c_int = 40;
pub const CLK_GOUT_SYSREG_PERI_PCLK: c_int = 41;
pub const CLK_GOUT_WDT0_PCLK: c_int = 42;
pub const CLK_GOUT_WDT1_PCLK: c_int = 43;
// CMU_FSYS
pub const CLK_MOUT_FSYS_BUS_USER: c_int = 1;
pub const CLK_MOUT_FSYS_MMC_CARD_USER: c_int = 2;
pub const CLK_MOUT_FSYS_MMC_EMBD_USER: c_int = 3;
pub const CLK_MOUT_FSYS_MMC_SDIO_USER: c_int = 4;
pub const CLK_GOUT_MMC_CARD_ACLK: c_int = 5;
pub const CLK_GOUT_MMC_CARD_SDCLKIN: c_int = 6;
pub const CLK_GOUT_MMC_EMBD_ACLK: c_int = 7;
pub const CLK_GOUT_MMC_EMBD_SDCLKIN: c_int = 8;
pub const CLK_GOUT_MMC_SDIO_ACLK: c_int = 9;
pub const CLK_GOUT_MMC_SDIO_SDCLKIN: c_int = 10;
pub const CLK_MOUT_FSYS_USB30DRD_USER: c_int = 11;
pub const CLK_MOUT_USB_PLL: c_int = 12;
pub const CLK_FOUT_USB_PLL: c_int = 13;
pub const CLK_FSYS_USB20PHY_CLKCORE: c_int = 14;
pub const CLK_FSYS_USB30DRD_ACLK_20PHYCTRL: c_int = 15;
pub const CLK_FSYS_USB30DRD_ACLK_30PHYCTRL_0: c_int = 16;
pub const CLK_FSYS_USB30DRD_ACLK_30PHYCTRL_1: c_int = 17;
pub const CLK_FSYS_USB30DRD_BUS_CLK_EARLY: c_int = 18;
pub const CLK_FSYS_USB30DRD_REF_CLK: c_int = 19;
