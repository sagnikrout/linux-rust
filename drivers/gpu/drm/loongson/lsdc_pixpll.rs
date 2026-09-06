//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/loongson/lsdc_pixpll.h
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
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

//
// Loongson Pixel PLL hardware structure
//
// refclk: reference frequency, 100 MHz from external oscillator
// outclk: output frequency desired.
//
// L1       Fref                      Fvco     L2
// refclk   +-----------+      +------------------+      +---------+   outclk
// ---+---> | Prescaler | ---> | Clock Multiplier | ---> | divider | -------->
// |     +-----------+      +------------------+      +---------+     ^
// |           ^                      ^                    ^          |
// |           |                      |                    |          |
// |        div_ref                 loopc               div_out       |
// |                                                                  |
// +---- bypass (bypass above software configurable clock if set) ----+
//
// outclk = refclk / div_ref * loopc / div_out;
//
// sel_out: PLL clock output selector(enable).
//
// If sel_out == 1, then enable output clock (turn On);
// If sel_out == 0, then disable output clock (turn Off);
//
// PLL working requirements:
//
// 1) 20 MHz <= refclk / div_ref <= 40Mhz
// 2) 1.2 GHz <= refclk /div_out * loopc <= 3.2 Ghz
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_pixpll_parms {
    pub ref_clock: c_uint,
    pub div_ref: c_uint,
    pub loopc: c_uint,
    pub div_out: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_pixpll_funcs {
    pub this): *const *const *const int (setup)(struct lsdc_pixpll,
    pub pout): *mut lsdc_pixpll_parms,
    pub pin): *const lsdc_pixpll_parms,
    pub this): *const *const *const unsigned int (get_rate)(struct lsdc_pixpll,
    pub printer): *mut drm_printer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_pixpll {
    pub funcs: *const lsdc_pixpll_funcs,
    pub ddev: *mut drm_device,
// PLL register offset
    pub reg_base: u32,
// PLL register size in bytes
    pub reg_size: u32,
    pub mmio: *mut void __iomem,
    pub priv: *mut lsdc_pixpll_parms,
}
