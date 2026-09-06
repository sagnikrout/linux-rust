//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/net-cw1200.h
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
// Copyright (C) ST-Ericsson SA 2011
//
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@stericsson.com>
//

// Macro flag: #define CW1200_PLAT_H_INCLUDED
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_platform_data_spi {
    pub /: *mut *mut u8 spi_bits_per_word; / REQUIRED,
    pub /: *mut *mut u16 ref_clk; / REQUIRED (in KHz),
// All others are optional
    pub have_5ghz: bool,
    pub /: *mut *mut bool enable); / Control 3v3 / 1v8 supply,
    pub /: *mut *mut bool enable); / Control CLK32K,
    pub /: *const *const *const u8 macaddr; / if NULL, use cw1200_mac_template module parameter,
    pub /: *const *const *const char sdd_file; / if NULL, will use default for detected hw type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_platform_data_sdio {
    pub /: *mut *mut u16 ref_clk; / REQUIRED (in KHz),
// All others are optional
    pub have_5ghz: bool,
    pub /: *mut *mut bool no_nptb; / SDIO hardware does not support non-power-of-2-blocksizes,
    pub /: *mut *mut int irq; / IRQ line or 0 to use SDIO IRQ,
    pub /: *mut *mut bool enable); / Control 3v3 / 1v8 supply,
    pub /: *mut *mut bool enable); / Control CLK32K,
    pub /: *const *const *const u8 macaddr; / if NULL, use cw1200_mac_template module parameter,
    pub /: *const *const *const char sdd_file; / if NULL, will use default for detected hw type,
}

// An example of SPI support in your board setup file:
//
// An example of SDIO support in your board setup file:
//
extern "C" {
    pub fn cw1200_sdio_set_platform_data(pdata: *mut cw1200_platform_data_sdio) -> void __init;
}
