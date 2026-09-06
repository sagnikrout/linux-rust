//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/microchip,mpfs-clock.h
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
// Daire McNamara,<daire.mcnamara@microchip.com>
// Copyright (C) 2020-2022 Microchip Technology Inc.  All rights reserved.
//
pub const CLK_CPU: c_int = 0;
pub const CLK_AXI: c_int = 1;
pub const CLK_AHB: c_int = 2;
pub const CLK_ENVM: c_int = 3;
pub const CLK_MAC0: c_int = 4;
pub const CLK_MAC1: c_int = 5;
pub const CLK_MMC: c_int = 6;
pub const CLK_TIMER: c_int = 7;
pub const CLK_MMUART0: c_int = 8;
pub const CLK_MMUART1: c_int = 9;
pub const CLK_MMUART2: c_int = 10;
pub const CLK_MMUART3: c_int = 11;
pub const CLK_MMUART4: c_int = 12;
pub const CLK_SPI0: c_int = 13;
pub const CLK_SPI1: c_int = 14;
pub const CLK_I2C0: c_int = 15;
pub const CLK_I2C1: c_int = 16;
pub const CLK_CAN0: c_int = 17;
pub const CLK_CAN1: c_int = 18;
pub const CLK_USB: c_int = 19;
pub const CLK_RESERVED: c_int = 20;
pub const CLK_RTC: c_int = 21;
pub const CLK_QSPI: c_int = 22;
pub const CLK_GPIO0: c_int = 23;
pub const CLK_GPIO1: c_int = 24;
pub const CLK_GPIO2: c_int = 25;
pub const CLK_DDRC: c_int = 26;
pub const CLK_FIC0: c_int = 27;
pub const CLK_FIC1: c_int = 28;
pub const CLK_FIC2: c_int = 29;
pub const CLK_FIC3: c_int = 30;
pub const CLK_ATHENA: c_int = 31;
pub const CLK_CFM: c_int = 32;
pub const CLK_RTCREF: c_int = 33;
pub const CLK_MSSPLL: c_int = 34;
pub const CLK_MSSPLL0: c_int = 34;
pub const CLK_MSSPLL1: c_int = 35;
pub const CLK_MSSPLL2: c_int = 36;
pub const CLK_MSSPLL3: c_int = 37;
// 38 is reserved for MSS PLL internals
// Clock Conditioning Circuitry Clock IDs
pub const CLK_CCC_PLL0: c_int = 0;
pub const CLK_CCC_PLL1: c_int = 1;
pub const CLK_CCC_DLL0: c_int = 2;
pub const CLK_CCC_DLL1: c_int = 3;
pub const CLK_CCC_PLL0_OUT0: c_int = 4;
pub const CLK_CCC_PLL0_OUT1: c_int = 5;
pub const CLK_CCC_PLL0_OUT2: c_int = 6;
pub const CLK_CCC_PLL0_OUT3: c_int = 7;
pub const CLK_CCC_PLL1_OUT0: c_int = 8;
pub const CLK_CCC_PLL1_OUT1: c_int = 9;
pub const CLK_CCC_PLL1_OUT2: c_int = 10;
pub const CLK_CCC_PLL1_OUT3: c_int = 11;
pub const CLK_CCC_DLL0_OUT0: c_int = 12;
pub const CLK_CCC_DLL0_OUT1: c_int = 13;
pub const CLK_CCC_DLL1_OUT0: c_int = 14;
pub const CLK_CCC_DLL1_OUT1: c_int = 15;
