//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8804.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// wm8804.h  --  WM8804 S/PDIF transceiver driver
//
// Copyright 2010 Wolfson Microelectronics plc
//
// Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
//

//
// Register values.
//
pub const WM8804_RST_DEVID1: c_uint = 0x00;
pub const WM8804_DEVID2: c_uint = 0x01;
pub const WM8804_DEVREV: c_uint = 0x02;
pub const WM8804_PLL1: c_uint = 0x03;
pub const WM8804_PLL2: c_uint = 0x04;
pub const WM8804_PLL3: c_uint = 0x05;
pub const WM8804_PLL4: c_uint = 0x06;
pub const WM8804_PLL5: c_uint = 0x07;
pub const WM8804_PLL6: c_uint = 0x08;
pub const WM8804_SPDMODE: c_uint = 0x09;
pub const WM8804_INTMASK: c_uint = 0x0A;
pub const WM8804_INTSTAT: c_uint = 0x0B;
pub const WM8804_SPDSTAT: c_uint = 0x0C;
pub const WM8804_RXCHAN1: c_uint = 0x0D;
pub const WM8804_RXCHAN2: c_uint = 0x0E;
pub const WM8804_RXCHAN3: c_uint = 0x0F;
pub const WM8804_RXCHAN4: c_uint = 0x10;
pub const WM8804_RXCHAN5: c_uint = 0x11;
pub const WM8804_SPDTX1: c_uint = 0x12;
pub const WM8804_SPDTX2: c_uint = 0x13;
pub const WM8804_SPDTX3: c_uint = 0x14;
pub const WM8804_SPDTX4: c_uint = 0x15;
pub const WM8804_SPDTX5: c_uint = 0x16;
pub const WM8804_GPO0: c_uint = 0x17;
pub const WM8804_GPO1: c_uint = 0x18;
pub const WM8804_GPO2: c_uint = 0x1A;
pub const WM8804_AIFTX: c_uint = 0x1B;
pub const WM8804_AIFRX: c_uint = 0x1C;
pub const WM8804_SPDRX1: c_uint = 0x1D;
pub const WM8804_PWRDN: c_uint = 0x1E;
pub const WM8804_REGISTER_COUNT: c_int = 30;
pub const WM8804_MAX_REGISTER: c_uint = 0x1E;
pub const WM8804_TX_CLKSRC_MCLK: c_int = 1;
pub const WM8804_TX_CLKSRC_PLL: c_int = 2;
pub const WM8804_CLKOUT_SRC_CLK1: c_int = 3;
pub const WM8804_CLKOUT_SRC_OSCCLK: c_int = 4;
pub const WM8804_CLKOUT_DIV: c_int = 1;
pub const WM8804_MCLK_DIV: c_int = 2;
pub const WM8804_MCLKDIV_256FS: c_int = 0;
pub const WM8804_MCLKDIV_128FS: c_int = 1;
extern "C" {
    pub fn wm8804_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn wm8804_remove(dev: *mut device);
}
