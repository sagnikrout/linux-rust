//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/bitmain,bm1880-reset.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2018 Bitmain Ltd.
// Copyright (c) 2019 Linaro Ltd.
//
pub const BM1880_RST_MAIN_AP: c_int = 0;
pub const BM1880_RST_SECOND_AP: c_int = 1;
pub const BM1880_RST_DDR: c_int = 2;
pub const BM1880_RST_VIDEO: c_int = 3;
pub const BM1880_RST_JPEG: c_int = 4;
pub const BM1880_RST_VPP: c_int = 5;
pub const BM1880_RST_GDMA: c_int = 6;
pub const BM1880_RST_AXI_SRAM: c_int = 7;
pub const BM1880_RST_TPU: c_int = 8;
pub const BM1880_RST_USB: c_int = 9;
pub const BM1880_RST_ETH0: c_int = 10;
pub const BM1880_RST_ETH1: c_int = 11;
pub const BM1880_RST_NAND: c_int = 12;
pub const BM1880_RST_EMMC: c_int = 13;
pub const BM1880_RST_SD: c_int = 14;
pub const BM1880_RST_SDMA: c_int = 15;
pub const BM1880_RST_I2S0: c_int = 16;
pub const BM1880_RST_I2S1: c_int = 17;
pub const BM1880_RST_UART0_1_CLK: c_int = 18;
pub const BM1880_RST_UART0_1_ACLK: c_int = 19;
pub const BM1880_RST_UART2_3_CLK: c_int = 20;
pub const BM1880_RST_UART2_3_ACLK: c_int = 21;
pub const BM1880_RST_MINER: c_int = 22;
pub const BM1880_RST_I2C0: c_int = 23;
pub const BM1880_RST_I2C1: c_int = 24;
pub const BM1880_RST_I2C2: c_int = 25;
pub const BM1880_RST_I2C3: c_int = 26;
pub const BM1880_RST_I2C4: c_int = 27;
pub const BM1880_RST_PWM0: c_int = 28;
pub const BM1880_RST_PWM1: c_int = 29;
pub const BM1880_RST_PWM2: c_int = 30;
pub const BM1880_RST_PWM3: c_int = 31;
pub const BM1880_RST_SPI: c_int = 32;
pub const BM1880_RST_GPIO0: c_int = 33;
pub const BM1880_RST_GPIO1: c_int = 34;
pub const BM1880_RST_GPIO2: c_int = 35;
pub const BM1880_RST_EFUSE: c_int = 36;
pub const BM1880_RST_WDT: c_int = 37;
pub const BM1880_RST_AHB_ROM: c_int = 38;
pub const BM1880_RST_SPIC: c_int = 39;
