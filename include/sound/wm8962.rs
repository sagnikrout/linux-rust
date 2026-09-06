//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm8962.h
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
// wm8962.h  --  WM8962 Soc Audio driver platform data
//
pub const WM8962_MAX_GPIO: c_int = 6;
// Use to set GPIO default values to zero
pub const WM8962_GPIO_SET: c_uint = 0x10000;
pub const WM8962_GPIO_FN_CLKOUT: c_int = 0;
pub const WM8962_GPIO_FN_LOGIC: c_int = 1;
pub const WM8962_GPIO_FN_SDOUT: c_int = 2;
pub const WM8962_GPIO_FN_IRQ: c_int = 3;
pub const WM8962_GPIO_FN_THERMAL: c_int = 4;
pub const WM8962_GPIO_FN_PLL2_LOCK: c_int = 6;
pub const WM8962_GPIO_FN_PLL3_LOCK: c_int = 7;
pub const WM8962_GPIO_FN_FLL_LOCK: c_int = 9;
pub const WM8962_GPIO_FN_DRC_ACT: c_int = 10;
pub const WM8962_GPIO_FN_WSEQ_DONE: c_int = 11;
pub const WM8962_GPIO_FN_ALC_NG_ACT: c_int = 12;
pub const WM8962_GPIO_FN_ALC_PEAK_LIMIT: c_int = 13;
pub const WM8962_GPIO_FN_ALC_SATURATION: c_int = 14;
pub const WM8962_GPIO_FN_ALC_LEVEL_THR: c_int = 15;
pub const WM8962_GPIO_FN_ALC_LEVEL_LOCK: c_int = 16;
pub const WM8962_GPIO_FN_FIFO_ERR: c_int = 17;
pub const WM8962_GPIO_FN_OPCLK: c_int = 18;
pub const WM8962_GPIO_FN_DMICCLK: c_int = 19;
pub const WM8962_GPIO_FN_DMICDAT: c_int = 20;
pub const WM8962_GPIO_FN_MICD: c_int = 21;
pub const WM8962_GPIO_FN_MICSCD: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8962_pdata {
    pub mclk: *mut clk,
    pub gpio_base: c_int,
    pub gpio_init: [u32; WM8962_MAX_GPIO],
// Setup for microphone detection, raw value to be written to
// R48(0x30) - only microphone related bits will be updated.
// Detection may be enabled here for use with signals brought
// out on the GPIOs.
    pub mic_cfg: u32,
    pub irq_active_low: bool,
    pub /: *mut *mut bool spk_mono; / Speaker outputs tied together as mono,
//
// This flag should be set if one or both IN4 inputs is wired
// in a DC measurement configuration.
//
    pub in4_dc_measure: bool,
}
