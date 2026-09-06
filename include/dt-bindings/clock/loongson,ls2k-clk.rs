//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/loongson,ls2k-clk.h
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
// Author: Yinbo Zhu <zhuyinbo@loongson.cn>
// Copyright (C) 2022-2023 Loongson Technology Corporation Limited
//
pub const LOONGSON2_REF_100M: c_int = 0;
pub const LOONGSON2_NODE_PLL: c_int = 1;
pub const LOONGSON2_DDR_PLL: c_int = 2;
pub const LOONGSON2_DC_PLL: c_int = 3;
pub const LOONGSON2_PIX0_PLL: c_int = 4;
pub const LOONGSON2_PIX1_PLL: c_int = 5;
pub const LOONGSON2_NODE_CLK: c_int = 6;
pub const LOONGSON2_HDA_CLK: c_int = 7;
pub const LOONGSON2_GPU_CLK: c_int = 8;
pub const LOONGSON2_DDR_CLK: c_int = 9;
pub const LOONGSON2_GMAC_CLK: c_int = 10;
pub const LOONGSON2_DC_CLK: c_int = 11;
pub const LOONGSON2_APB_CLK: c_int = 12;
pub const LOONGSON2_USB_CLK: c_int = 13;
pub const LOONGSON2_SATA_CLK: c_int = 14;
pub const LOONGSON2_PIX0_CLK: c_int = 15;
pub const LOONGSON2_PIX1_CLK: c_int = 16;
pub const LOONGSON2_BOOT_CLK: c_int = 17;
pub const LOONGSON2_OUT0_GATE: c_int = 18;
pub const LOONGSON2_GMAC_GATE: c_int = 19;
pub const LOONGSON2_RIO_GATE: c_int = 20;
pub const LOONGSON2_DC_GATE: c_int = 21;
pub const LOONGSON2_GPU_GATE: c_int = 22;
pub const LOONGSON2_DDR_GATE: c_int = 23;
pub const LOONGSON2_HDA_GATE: c_int = 24;
pub const LOONGSON2_NODE_GATE: c_int = 25;
pub const LOONGSON2_EMMC_GATE: c_int = 26;
pub const LOONGSON2_PIX0_GATE: c_int = 27;
pub const LOONGSON2_PIX1_GATE: c_int = 28;
pub const LOONGSON2_OUT0_CLK: c_int = 29;
pub const LOONGSON2_RIO_CLK: c_int = 30;
pub const LOONGSON2_EMMC_CLK: c_int = 31;
pub const LOONGSON2_DES_CLK: c_int = 32;
pub const LOONGSON2_I2S_CLK: c_int = 33;
pub const LOONGSON2_MISC_CLK: c_int = 34;
pub const LS2K0300_CLK_STABLE: c_int = 0;
pub const LS2K0300_NODE_PLL: c_int = 1;
pub const LS2K0300_DDR_PLL: c_int = 2;
pub const LS2K0300_PIX_PLL: c_int = 3;
pub const LS2K0300_CLK_THSENS: c_int = 4;
pub const LS2K0300_CLK_NODE_DIV: c_int = 5;
pub const LS2K0300_CLK_NODE_PLL_GATE: c_int = 6;
pub const LS2K0300_CLK_NODE_SCALE: c_int = 7;
pub const LS2K0300_CLK_NODE_GATE: c_int = 8;
pub const LS2K0300_CLK_GMAC_DIV: c_int = 9;
pub const LS2K0300_CLK_GMAC_GATE: c_int = 10;
pub const LS2K0300_CLK_I2S_DIV: c_int = 11;
pub const LS2K0300_CLK_I2S_SCALE: c_int = 12;
pub const LS2K0300_CLK_I2S_GATE: c_int = 13;
pub const LS2K0300_CLK_DDR_DIV: c_int = 14;
pub const LS2K0300_CLK_DDR_GATE: c_int = 15;
pub const LS2K0300_CLK_NET_DIV: c_int = 16;
pub const LS2K0300_CLK_NET_GATE: c_int = 17;
pub const LS2K0300_CLK_DEV_DIV: c_int = 18;
pub const LS2K0300_CLK_DEV_GATE: c_int = 19;
pub const LS2K0300_CLK_PIX_DIV: c_int = 20;
pub const LS2K0300_CLK_PIX_PLL_GATE: c_int = 21;
pub const LS2K0300_CLK_PIX_SCALE: c_int = 22;
pub const LS2K0300_CLK_PIX_GATE: c_int = 23;
pub const LS2K0300_CLK_GMACBP_DIV: c_int = 24;
pub const LS2K0300_CLK_GMACBP_GATE: c_int = 25;
pub const LS2K0300_CLK_USB_SCALE: c_int = 26;
pub const LS2K0300_CLK_USB_GATE: c_int = 27;
pub const LS2K0300_CLK_APB_SCALE: c_int = 28;
pub const LS2K0300_CLK_APB_GATE: c_int = 29;
pub const LS2K0300_CLK_BOOT_SCALE: c_int = 30;
pub const LS2K0300_CLK_BOOT_GATE: c_int = 31;
pub const LS2K0300_CLK_SDIO_SCALE: c_int = 32;
pub const LS2K0300_CLK_SDIO_GATE: c_int = 33;
pub const LS2K0300_CLK_GMAC_IN: c_int = 34;
