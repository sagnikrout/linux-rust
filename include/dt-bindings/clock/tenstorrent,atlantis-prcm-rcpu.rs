//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tenstorrent,atlantis-prcm-rcpu.h
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
// Tenstorrent Atlantis PRCM Clock and Reset Indices
//
// Copyright (c) 2026 Tenstorrent
//
// RCPU Domain Clock IDs
//
pub const CLK_RCPU_PLL: c_int = 0;
pub const CLK_RCPU_ROOT: c_int = 1;
pub const CLK_RCPU_DIV2: c_int = 2;
pub const CLK_RCPU_DIV4: c_int = 3;
pub const CLK_RCPU_RTC: c_int = 4;
pub const CLK_SMNDMA0_ACLK: c_int = 5;
pub const CLK_SMNDMA1_ACLK: c_int = 6;
pub const CLK_WDT0_PCLK: c_int = 7;
pub const CLK_WDT1_PCLK: c_int = 8;
pub const CLK_TIMER_PCLK: c_int = 9;
pub const CLK_PVTC_PCLK: c_int = 10;
pub const CLK_PMU_PCLK: c_int = 11;
pub const CLK_MAILBOX_HCLK: c_int = 12;
pub const CLK_SEC_SPACC_HCLK: c_int = 13;
pub const CLK_SEC_OTP_HCLK: c_int = 14;
pub const CLK_TRNG_PCLK: c_int = 15;
pub const CLK_SEC_CRC_HCLK: c_int = 16;
pub const CLK_SMN_HCLK: c_int = 17;
pub const CLK_AHB0_HCLK: c_int = 18;
pub const CLK_SMN_PCLK: c_int = 19;
pub const CLK_SMN_CLK: c_int = 20;
pub const CLK_SCRATCHPAD_CLK: c_int = 21;
pub const CLK_RCPU_CORE_CLK: c_int = 22;
pub const CLK_RCPU_ROM_CLK: c_int = 23;
pub const CLK_OTP_LOAD_CLK: c_int = 24;
pub const CLK_NOC_PLL: c_int = 25;
pub const CLK_NOCC_CLK: c_int = 26;
pub const CLK_NOCC_DIV2: c_int = 27;
pub const CLK_NOCC_DIV4: c_int = 28;
pub const CLK_NOCC_RTC: c_int = 29;
pub const CLK_NOCC_CAN: c_int = 30;
pub const CLK_QSPI_SCLK: c_int = 31;
pub const CLK_QSPI_HCLK: c_int = 32;
pub const CLK_I2C0_PCLK: c_int = 33;
pub const CLK_I2C1_PCLK: c_int = 34;
pub const CLK_I2C2_PCLK: c_int = 35;
pub const CLK_I2C3_PCLK: c_int = 36;
pub const CLK_I2C4_PCLK: c_int = 37;
pub const CLK_UART0_PCLK: c_int = 38;
pub const CLK_UART1_PCLK: c_int = 39;
pub const CLK_UART2_PCLK: c_int = 40;
pub const CLK_UART3_PCLK: c_int = 41;
pub const CLK_UART4_PCLK: c_int = 42;
pub const CLK_SPI0_PCLK: c_int = 43;
pub const CLK_SPI1_PCLK: c_int = 44;
pub const CLK_SPI2_PCLK: c_int = 45;
pub const CLK_SPI3_PCLK: c_int = 46;
pub const CLK_GPIO_PCLK: c_int = 47;
pub const CLK_CAN0_HCLK: c_int = 48;
pub const CLK_CAN0_CLK: c_int = 49;
pub const CLK_CAN1_HCLK: c_int = 50;
pub const CLK_CAN1_CLK: c_int = 51;
pub const CLK_CAN0_TIMER_CLK: c_int = 52;
pub const CLK_CAN1_TIMER_CLK: c_int = 53;
// RCPU domain reset
pub const RST_SMNDMA0: c_int = 0;
pub const RST_SMNDMA1: c_int = 1;
pub const RST_WDT0: c_int = 2;
pub const RST_WDT1: c_int = 3;
pub const RST_TMR: c_int = 4;
pub const RST_PVTC: c_int = 5;
pub const RST_PMU: c_int = 6;
pub const RST_MAILBOX: c_int = 7;
pub const RST_SPACC: c_int = 8;
pub const RST_OTP: c_int = 9;
pub const RST_TRNG: c_int = 10;
pub const RST_CRC: c_int = 11;
pub const RST_QSPI: c_int = 12;
pub const RST_I2C0: c_int = 13;
pub const RST_I2C1: c_int = 14;
pub const RST_I2C2: c_int = 15;
pub const RST_I2C3: c_int = 16;
pub const RST_I2C4: c_int = 17;
pub const RST_UART0: c_int = 18;
pub const RST_UART1: c_int = 19;
pub const RST_UART2: c_int = 20;
pub const RST_UART3: c_int = 21;
pub const RST_UART4: c_int = 22;
pub const RST_SPI0: c_int = 23;
pub const RST_SPI1: c_int = 24;
pub const RST_SPI2: c_int = 25;
pub const RST_SPI3: c_int = 26;
pub const RST_GPIO: c_int = 27;
pub const RST_CAN0: c_int = 28;
pub const RST_CAN1: c_int = 29;
pub const RST_I2S0: c_int = 30;
pub const RST_I2S1: c_int = 31;
