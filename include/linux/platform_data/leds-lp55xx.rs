//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/leds-lp55xx.h
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
// LP55XX Platform Data Header
//
// Copyright (C) 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//
// Derived from leds-lp5521.h, leds-lp5523.h
//

// Clock configuration
pub const LP55XX_CLOCK_AUTO: c_int = 0;
pub const LP55XX_CLOCK_INT: c_int = 1;
pub const LP55XX_CLOCK_EXT: c_int = 2;
pub const LP55XX_MAX_GROUPED_CHAN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_led_config {
    pub name: *const c_char,
    pub default_trigger: *const c_char,
    pub chan_nr: u8,
    pub /: *mut *mut u8 led_current; / mA x10, 0 if led is not connected,
    pub max_current: u8,
    pub num_colors: c_int,
    pub max_channel: c_uint,
    pub color_id: [c_int; LED_COLOR_ID_MAX],
    pub output_num: [c_int; LED_COLOR_ID_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_predef_pattern {
    pub r: *const u8,
    pub g: *const u8,
    pub b: *const u8,
    pub size_r: u8,
    pub size_g: u8,
    pub size_b: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8501_pwr_sel {
    LP8501_ALL_VDD,		/* D1~9 are connected to VDD */
    LP8501_6VDD_3VOUT,	/* D1~6 with VDD, D7~9 with VOUT */
    LP8501_3VDD_6VOUT,	/* D1~6 with VOUT, D7~9 with VDD */
    LP8501_ALL_VOUT,	/* D1~9 are connected to VOUT */
}

//
// struct lp55xx_platform_data
// @led_config        : Configurable led class device
// @num_channels      : Number of LED channels
// @label             : Used for naming LEDs
// @clock_mode        : Input clock mode. LP55XX_CLOCK_AUTO or _INT or _EXT
// @setup_resources   : Platform specific function before enabling the chip
// @release_resources : Platform specific function after  disabling the chip
// @enable_gpiod      : enable GPIO descriptor
// @patterns          : Predefined pattern data for RGB channels
// @num_patterns      : Number of patterns
// @update_config     : Value of CONFIG register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp55xx_platform_data {
// LED channel configuration
    pub led_config: *mut lp55xx_led_config,
    pub num_channels: u8,
    pub label: *const c_char,
// Clock configuration
    pub clock_mode: u8,
// Charge pump mode
    pub charge_pump_mode: u32,
// optional enable GPIO
    pub enable_gpiod: *mut gpio_desc,
// Predefined pattern data
    pub patterns: *mut lp55xx_predef_pattern,
    pub num_patterns: c_uint,
// LP8501 specific
    pub pwr_sel: lp8501_pwr_sel,
}
