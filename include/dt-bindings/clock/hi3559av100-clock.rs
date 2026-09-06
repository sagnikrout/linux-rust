//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hi3559av100-clock.h
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


// SPDX-License-Identifier: GPL-2.0-or-later OR BSD-2-Clause
//
// Copyright (c) 2019-2020, Huawei Tech. Co., Ltd.
//
// Author: Dongjiu Geng <gengdongjiu@huawei.com>
//
// fixed   rate
pub const HI3559AV100_FIXED_1188M: c_int = 1;
pub const HI3559AV100_FIXED_1000M: c_int = 2;
pub const HI3559AV100_FIXED_842M: c_int = 3;
pub const HI3559AV100_FIXED_792M: c_int = 4;
pub const HI3559AV100_FIXED_750M: c_int = 5;
pub const HI3559AV100_FIXED_710M: c_int = 6;
pub const HI3559AV100_FIXED_680M: c_int = 7;
pub const HI3559AV100_FIXED_667M: c_int = 8;
pub const HI3559AV100_FIXED_631M: c_int = 9;
pub const HI3559AV100_FIXED_600M: c_int = 10;
pub const HI3559AV100_FIXED_568M: c_int = 11;
pub const HI3559AV100_FIXED_500M: c_int = 12;
pub const HI3559AV100_FIXED_475M: c_int = 13;
pub const HI3559AV100_FIXED_428M: c_int = 14;
pub const HI3559AV100_FIXED_400M: c_int = 15;
pub const HI3559AV100_FIXED_396M: c_int = 16;
pub const HI3559AV100_FIXED_300M: c_int = 17;
pub const HI3559AV100_FIXED_250M: c_int = 18;
pub const HI3559AV100_FIXED_198M: c_int = 19;
pub const HI3559AV100_FIXED_187p5M: c_int = 20;
pub const HI3559AV100_FIXED_150M: c_int = 21;
pub const HI3559AV100_FIXED_148p5M: c_int = 22;
pub const HI3559AV100_FIXED_125M: c_int = 23;
pub const HI3559AV100_FIXED_107M: c_int = 24;
pub const HI3559AV100_FIXED_100M: c_int = 25;
pub const HI3559AV100_FIXED_99M: c_int = 26;
pub const HI3559AV100_FIXED_74p25M: c_int = 27;
pub const HI3559AV100_FIXED_72M: c_int = 28;
pub const HI3559AV100_FIXED_60M: c_int = 29;
pub const HI3559AV100_FIXED_54M: c_int = 30;
pub const HI3559AV100_FIXED_50M: c_int = 31;
pub const HI3559AV100_FIXED_49p5M: c_int = 32;
pub const HI3559AV100_FIXED_37p125M: c_int = 33;
pub const HI3559AV100_FIXED_36M: c_int = 34;
pub const HI3559AV100_FIXED_32p4M: c_int = 35;
pub const HI3559AV100_FIXED_27M: c_int = 36;
pub const HI3559AV100_FIXED_25M: c_int = 37;
pub const HI3559AV100_FIXED_24M: c_int = 38;
pub const HI3559AV100_FIXED_12M: c_int = 39;
pub const HI3559AV100_FIXED_3M: c_int = 40;
pub const HI3559AV100_FIXED_1p6M: c_int = 41;
pub const HI3559AV100_FIXED_400K: c_int = 42;
pub const HI3559AV100_FIXED_100K: c_int = 43;
pub const HI3559AV100_FIXED_200M: c_int = 44;
pub const HI3559AV100_FIXED_75M: c_int = 75;
pub const HI3559AV100_I2C0_CLK: c_int = 50;
pub const HI3559AV100_I2C1_CLK: c_int = 51;
pub const HI3559AV100_I2C2_CLK: c_int = 52;
pub const HI3559AV100_I2C3_CLK: c_int = 53;
pub const HI3559AV100_I2C4_CLK: c_int = 54;
pub const HI3559AV100_I2C5_CLK: c_int = 55;
pub const HI3559AV100_I2C6_CLK: c_int = 56;
pub const HI3559AV100_I2C7_CLK: c_int = 57;
pub const HI3559AV100_I2C8_CLK: c_int = 58;
pub const HI3559AV100_I2C9_CLK: c_int = 59;
pub const HI3559AV100_I2C10_CLK: c_int = 60;
pub const HI3559AV100_I2C11_CLK: c_int = 61;
pub const HI3559AV100_SPI0_CLK: c_int = 62;
pub const HI3559AV100_SPI1_CLK: c_int = 63;
pub const HI3559AV100_SPI2_CLK: c_int = 64;
pub const HI3559AV100_SPI3_CLK: c_int = 65;
pub const HI3559AV100_SPI4_CLK: c_int = 66;
pub const HI3559AV100_SPI5_CLK: c_int = 67;
pub const HI3559AV100_SPI6_CLK: c_int = 68;
pub const HI3559AV100_EDMAC_CLK: c_int = 69;
pub const HI3559AV100_EDMAC_AXICLK: c_int = 70;
pub const HI3559AV100_EDMAC1_CLK: c_int = 71;
pub const HI3559AV100_EDMAC1_AXICLK: c_int = 72;
pub const HI3559AV100_VDMAC_CLK: c_int = 73;
// mux clocks
pub const HI3559AV100_FMC_MUX: c_int = 80;
pub const HI3559AV100_SYSAPB_MUX: c_int = 81;
pub const HI3559AV100_UART_MUX: c_int = 82;
pub const HI3559AV100_SYSBUS_MUX: c_int = 83;
pub const HI3559AV100_A73_MUX: c_int = 84;
pub const HI3559AV100_MMC0_MUX: c_int = 85;
pub const HI3559AV100_MMC1_MUX: c_int = 86;
pub const HI3559AV100_MMC2_MUX: c_int = 87;
pub const HI3559AV100_MMC3_MUX: c_int = 88;
// gate    clocks
pub const HI3559AV100_FMC_CLK: c_int = 90;
pub const HI3559AV100_UART0_CLK: c_int = 91;
pub const HI3559AV100_UART1_CLK: c_int = 92;
pub const HI3559AV100_UART2_CLK: c_int = 93;
pub const HI3559AV100_UART3_CLK: c_int = 94;
pub const HI3559AV100_UART4_CLK: c_int = 95;
pub const HI3559AV100_MMC0_CLK: c_int = 96;
pub const HI3559AV100_MMC1_CLK: c_int = 97;
pub const HI3559AV100_MMC2_CLK: c_int = 98;
pub const HI3559AV100_MMC3_CLK: c_int = 99;
pub const HI3559AV100_ETH_CLK: c_int = 100;
pub const HI3559AV100_ETH_MACIF_CLK: c_int = 101;
pub const HI3559AV100_ETH1_CLK: c_int = 102;
pub const HI3559AV100_ETH1_MACIF_CLK: c_int = 103;
// complex
pub const HI3559AV100_MAC0_CLK: c_int = 110;
pub const HI3559AV100_MAC1_CLK: c_int = 111;
pub const HI3559AV100_SATA_CLK: c_int = 112;
pub const HI3559AV100_USB_CLK: c_int = 113;
pub const HI3559AV100_USB1_CLK: c_int = 114;
// pll clocks
pub const HI3559AV100_APLL_CLK: c_int = 250;
pub const HI3559AV100_GPLL_CLK: c_int = 251;
pub const HI3559AV100_CRG_NR_CLKS: c_int = 256;
pub const HI3559AV100_SHUB_SOURCE_SOC_24M: c_int = 0;
pub const HI3559AV100_SHUB_SOURCE_SOC_200M: c_int = 1;
pub const HI3559AV100_SHUB_SOURCE_SOC_300M: c_int = 2;
pub const HI3559AV100_SHUB_SOURCE_PLL: c_int = 3;
pub const HI3559AV100_SHUB_SOURCE_CLK: c_int = 4;
pub const HI3559AV100_SHUB_I2C0_CLK: c_int = 10;
pub const HI3559AV100_SHUB_I2C1_CLK: c_int = 11;
pub const HI3559AV100_SHUB_I2C2_CLK: c_int = 12;
pub const HI3559AV100_SHUB_I2C3_CLK: c_int = 13;
pub const HI3559AV100_SHUB_I2C4_CLK: c_int = 14;
pub const HI3559AV100_SHUB_I2C5_CLK: c_int = 15;
pub const HI3559AV100_SHUB_I2C6_CLK: c_int = 16;
pub const HI3559AV100_SHUB_I2C7_CLK: c_int = 17;
pub const HI3559AV100_SHUB_SPI_SOURCE_CLK: c_int = 20;
pub const HI3559AV100_SHUB_SPI4_SOURCE_CLK: c_int = 21;
pub const HI3559AV100_SHUB_SPI0_CLK: c_int = 22;
pub const HI3559AV100_SHUB_SPI1_CLK: c_int = 23;
pub const HI3559AV100_SHUB_SPI2_CLK: c_int = 24;
pub const HI3559AV100_SHUB_SPI3_CLK: c_int = 25;
pub const HI3559AV100_SHUB_SPI4_CLK: c_int = 26;
pub const HI3559AV100_SHUB_UART_CLK_32K: c_int = 30;
pub const HI3559AV100_SHUB_UART_SOURCE_CLK: c_int = 31;
pub const HI3559AV100_SHUB_UART_DIV_CLK: c_int = 32;
pub const HI3559AV100_SHUB_UART0_CLK: c_int = 33;
pub const HI3559AV100_SHUB_UART1_CLK: c_int = 34;
pub const HI3559AV100_SHUB_UART2_CLK: c_int = 35;
pub const HI3559AV100_SHUB_UART3_CLK: c_int = 36;
pub const HI3559AV100_SHUB_UART4_CLK: c_int = 37;
pub const HI3559AV100_SHUB_UART5_CLK: c_int = 38;
pub const HI3559AV100_SHUB_UART6_CLK: c_int = 39;
pub const HI3559AV100_SHUB_EDMAC_CLK: c_int = 40;
pub const HI3559AV100_SHUB_NR_CLKS: c_int = 50;
