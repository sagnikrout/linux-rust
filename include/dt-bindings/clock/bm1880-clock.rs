//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/bm1880-clock.h
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
// Device Tree binding constants for Bitmain BM1880 SoC
//
// Copyright (c) 2019 Linaro Ltd.
//
pub const BM1880_CLK_OSC: c_int = 0;
pub const BM1880_CLK_MPLL: c_int = 1;
pub const BM1880_CLK_SPLL: c_int = 2;
pub const BM1880_CLK_FPLL: c_int = 3;
pub const BM1880_CLK_DDRPLL: c_int = 4;
pub const BM1880_CLK_A53: c_int = 5;
pub const BM1880_CLK_50M_A53: c_int = 6;
pub const BM1880_CLK_AHB_ROM: c_int = 7;
pub const BM1880_CLK_AXI_SRAM: c_int = 8;
pub const BM1880_CLK_DDR_AXI: c_int = 9;
pub const BM1880_CLK_EFUSE: c_int = 10;
pub const BM1880_CLK_APB_EFUSE: c_int = 11;
pub const BM1880_CLK_AXI5_EMMC: c_int = 12;
pub const BM1880_CLK_EMMC: c_int = 13;
pub const BM1880_CLK_100K_EMMC: c_int = 14;
pub const BM1880_CLK_AXI5_SD: c_int = 15;
pub const BM1880_CLK_SD: c_int = 16;
pub const BM1880_CLK_100K_SD: c_int = 17;
pub const BM1880_CLK_500M_ETH0: c_int = 18;
pub const BM1880_CLK_AXI4_ETH0: c_int = 19;
pub const BM1880_CLK_500M_ETH1: c_int = 20;
pub const BM1880_CLK_AXI4_ETH1: c_int = 21;
pub const BM1880_CLK_AXI1_GDMA: c_int = 22;
pub const BM1880_CLK_APB_GPIO: c_int = 23;
pub const BM1880_CLK_APB_GPIO_INTR: c_int = 24;
pub const BM1880_CLK_GPIO_DB: c_int = 25;
pub const BM1880_CLK_AXI1_MINER: c_int = 26;
pub const BM1880_CLK_AHB_SF: c_int = 27;
pub const BM1880_CLK_SDMA_AXI: c_int = 28;
pub const BM1880_CLK_SDMA_AUD: c_int = 29;
pub const BM1880_CLK_APB_I2C: c_int = 30;
pub const BM1880_CLK_APB_WDT: c_int = 31;
pub const BM1880_CLK_APB_JPEG: c_int = 32;
pub const BM1880_CLK_JPEG_AXI: c_int = 33;
pub const BM1880_CLK_AXI5_NF: c_int = 34;
pub const BM1880_CLK_APB_NF: c_int = 35;
pub const BM1880_CLK_NF: c_int = 36;
pub const BM1880_CLK_APB_PWM: c_int = 37;
pub const BM1880_CLK_DIV_0_RV: c_int = 38;
pub const BM1880_CLK_DIV_1_RV: c_int = 39;
pub const BM1880_CLK_MUX_RV: c_int = 40;
pub const BM1880_CLK_RV: c_int = 41;
pub const BM1880_CLK_APB_SPI: c_int = 42;
pub const BM1880_CLK_TPU_AXI: c_int = 43;
pub const BM1880_CLK_DIV_UART_500M: c_int = 44;
pub const BM1880_CLK_UART_500M: c_int = 45;
pub const BM1880_CLK_APB_UART: c_int = 46;
pub const BM1880_CLK_APB_I2S: c_int = 47;
pub const BM1880_CLK_AXI4_USB: c_int = 48;
pub const BM1880_CLK_APB_USB: c_int = 49;
pub const BM1880_CLK_125M_USB: c_int = 50;
pub const BM1880_CLK_33K_USB: c_int = 51;
pub const BM1880_CLK_DIV_12M_USB: c_int = 52;
pub const BM1880_CLK_12M_USB: c_int = 53;
pub const BM1880_CLK_APB_VIDEO: c_int = 54;
pub const BM1880_CLK_VIDEO_AXI: c_int = 55;
pub const BM1880_CLK_VPP_AXI: c_int = 56;
pub const BM1880_CLK_APB_VPP: c_int = 57;
pub const BM1880_CLK_DIV_0_AXI1: c_int = 58;
pub const BM1880_CLK_DIV_1_AXI1: c_int = 59;
pub const BM1880_CLK_AXI1: c_int = 60;
pub const BM1880_CLK_AXI2: c_int = 61;
pub const BM1880_CLK_AXI3: c_int = 62;
pub const BM1880_CLK_AXI4: c_int = 63;
pub const BM1880_CLK_AXI5: c_int = 64;
pub const BM1880_CLK_DIV_0_AXI6: c_int = 65;
pub const BM1880_CLK_DIV_1_AXI6: c_int = 66;
pub const BM1880_CLK_MUX_AXI6: c_int = 67;
pub const BM1880_CLK_AXI6: c_int = 68;
pub const BM1880_NR_CLKS: c_int = 69;
