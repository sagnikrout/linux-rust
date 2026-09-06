//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/pinctrl-sg2002.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (C) 2024 Inochi Amaoto <inochiama@outlook.com>
//
// This file is generated from vendor pinout definition.
//

pub const PIN_AUD_AINL_MIC: c_int = 2;
pub const PIN_AUD_AOUTR: c_int = 4;
pub const PIN_SD0_CLK: c_int = 6;
pub const PIN_SD0_CMD: c_int = 7;
pub const PIN_SD0_D0: c_int = 8;
pub const PIN_SD0_D1: c_int = 10;
pub const PIN_SD0_D2: c_int = 11;
pub const PIN_SD0_D3: c_int = 12;
pub const PIN_SD0_CD: c_int = 14;
pub const PIN_SD0_PWR_EN: c_int = 15;
pub const PIN_SPK_EN: c_int = 17;
pub const PIN_UART0_TX: c_int = 18;
pub const PIN_UART0_RX: c_int = 19;
pub const PIN_EMMC_DAT2: c_int = 20;
pub const PIN_EMMC_CLK: c_int = 21;
pub const PIN_EMMC_DAT0: c_int = 22;
pub const PIN_EMMC_DAT3: c_int = 23;
pub const PIN_EMMC_CMD: c_int = 24;
pub const PIN_EMMC_DAT1: c_int = 25;
pub const PIN_JTAG_CPU_TMS: c_int = 26;
pub const PIN_JTAG_CPU_TCK: c_int = 27;
pub const PIN_IIC0_SCL: c_int = 28;
pub const PIN_IIC0_SDA: c_int = 29;
pub const PIN_AUX0: c_int = 30;
pub const PIN_GPIO_ZQ: c_int = 35;
pub const PIN_PWR_VBAT_DET: c_int = 38;
pub const PIN_PWR_RSTN: c_int = 39;
pub const PIN_PWR_SEQ1: c_int = 40;
pub const PIN_PWR_SEQ2: c_int = 41;
pub const PIN_PWR_WAKEUP0: c_int = 43;
pub const PIN_PWR_BUTTON1: c_int = 44;
pub const PIN_XTAL_XIN: c_int = 45;
pub const PIN_PWR_GPIO0: c_int = 47;
pub const PIN_PWR_GPIO1: c_int = 48;
pub const PIN_PWR_GPIO2: c_int = 49;
pub const PIN_SD1_D3: c_int = 51;
pub const PIN_SD1_D2: c_int = 52;
pub const PIN_SD1_D1: c_int = 53;
pub const PIN_SD1_D0: c_int = 54;
pub const PIN_SD1_CMD: c_int = 55;
pub const PIN_SD1_CLK: c_int = 56;
pub const PIN_PWM0_BUCK: c_int = 58;
pub const PIN_ADC1: c_int = 59;
pub const PIN_USB_VBUS_DET: c_int = 60;
pub const PIN_ETH_TXP: c_int = 62;
pub const PIN_ETH_TXM: c_int = 63;
pub const PIN_ETH_RXP: c_int = 64;
pub const PIN_ETH_RXM: c_int = 65;
pub const PIN_GPIO_RTX: c_int = 67;
pub const PIN_MIPIRX4N: c_int = 72;
pub const PIN_MIPIRX4P: c_int = 73;
pub const PIN_MIPIRX3N: c_int = 74;
pub const PIN_MIPIRX3P: c_int = 75;
pub const PIN_MIPIRX2N: c_int = 76;
pub const PIN_MIPIRX2P: c_int = 77;
pub const PIN_MIPIRX1N: c_int = 78;
pub const PIN_MIPIRX1P: c_int = 79;
pub const PIN_MIPIRX0N: c_int = 80;
pub const PIN_MIPIRX0P: c_int = 81;
pub const PIN_MIPI_TXM2: c_int = 83;
pub const PIN_MIPI_TXP2: c_int = 84;
pub const PIN_MIPI_TXM1: c_int = 85;
pub const PIN_MIPI_TXP1: c_int = 86;
pub const PIN_MIPI_TXM0: c_int = 87;
pub const PIN_MIPI_TXP0: c_int = 88;
