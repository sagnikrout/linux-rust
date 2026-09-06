//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds-lp3952.h
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
// LED driver for TI lp3952 controller
//
// Copyright (C) 2016, DAQRI, LLC.
// Author: Tony Makkiel <tony.makkiel@daqri.com>
//

pub const LP3952_CMD_REG_COUNT: c_int = 8;
pub const LP3952_BRIGHT_MAX: c_int = 4;
pub const LP3952_LABEL_MAX_LEN: c_int = 15;
pub const LP3952_REG_LED_CTRL: c_uint = 0x00;
pub const LP3952_REG_R1_BLNK_TIME_CTRL: c_uint = 0x01;
pub const LP3952_REG_R1_BLNK_CYCLE_CTRL: c_uint = 0x02;
pub const LP3952_REG_G1_BLNK_TIME_CTRL: c_uint = 0x03;
pub const LP3952_REG_G1_BLNK_CYCLE_CTRL: c_uint = 0x04;
pub const LP3952_REG_B1_BLNK_TIME_CTRL: c_uint = 0x05;
pub const LP3952_REG_B1_BLNK_CYCLE_CTRL: c_uint = 0x06;
pub const LP3952_REG_ENABLES: c_uint = 0x0B;
pub const LP3952_REG_PAT_GEN_CTRL: c_uint = 0x11;
pub const LP3952_REG_RGB1_MAX_I_CTRL: c_uint = 0x12;
pub const LP3952_REG_RGB2_MAX_I_CTRL: c_uint = 0x13;
pub const LP3952_REG_CMD_0: c_uint = 0x50;
pub const LP3952_REG_RESET: c_uint = 0x60;

pub const LP3952_LED_MASK_ALL: c_uint = 0x3f;
// Transition Time in ms
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3952_tt {
    TT0,
    TT55,
    TT110,
    TT221,
    TT422,
    TT885,
    TT1770,
    TT3539
}

// Command Execution Time in ms
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3952_cet {
    CET197,
    CET393,
    CET590,
    CET786,
    CET1180,
    CET1376,
    CET1573,
    CET1769,
    CET1966,
    CET2163,
    CET2359,
    CET2556,
    CET2763,
    CET2949,
    CET3146
}

// Max Current in %
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3952_colour_I_log_0 {
    I0,
    I7,
    I14,
    I21,
    I32,
    I46,
    I71,
    I100
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3952_leds {
    LP3952_BLUE_2,
    LP3952_GREEN_2,
    LP3952_RED_2,
    LP3952_BLUE_1,
    LP3952_GREEN_1,
    LP3952_RED_1,
    LP3952_LED_ALL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3952_ctrl_hdl {
    pub cdev: led_classdev,
    pub name: [c_char; LP3952_LABEL_MAX_LEN],
    pub channel: lp3952_leds,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrn_gen_cmd {
    pub tt:3: u16,
    pub b:3: u16,
    pub cet:4: u16,
    pub g:3: u16,
    pub r:3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3952_led_array {
    pub regmap: *mut regmap,
    pub client: *mut i2c_client,
    pub enable_gpio: *mut gpio_desc,
    pub leds: [lp3952_ctrl_hdl; LP3952_LED_ALL],
}
