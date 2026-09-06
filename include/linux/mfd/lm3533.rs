//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lm3533.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// lm3533.h -- LM3533 interface
//
// Copyright (C) 2011-2012 Texas Instruments
//
// Author: Johan Hovold <jhovold@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3533 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub hwen: *mut gpio_desc,
    pub irq: c_int,
    pub have_als:1: unsigned,
    pub have_backlights:1: unsigned,
    pub have_leds:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3533_ctrlbank {
    pub lm3533: *mut lm3533,
    pub dev: *mut device,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3533_als_platform_data {
    pub /: *mut *mut unsigned pwm_mode:1; / PWM input mode (default analog),
    pub /: *mut *mut u8 r_select; / 1 - 127 (ignored in PWM-mode),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3533_bl_platform_data {
    pub name: *mut c_char,
    pub /: *mut *mut u16 max_current; / 5000 - 29800 uA (800 uA step),
    pub /: *mut *mut u8 default_brightness; / 0 - 255,
    pub /: *mut *mut u8 pwm; / 0 - 0x3f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3533_led_platform_data {
    pub name: *mut c_char,
    pub default_trigger: *const c_char,
    pub /: *mut *mut u16 max_current; / 5000 - 29800 uA (800 uA step),
    pub /: *mut *mut u8 pwm; / 0 - 0x3f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3533_boost_freq {
    LM3533_BOOST_FREQ_500KHZ,
    LM3533_BOOST_FREQ_1000KHZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3533_boost_ovp {
    LM3533_BOOST_OVP_16V,
    LM3533_BOOST_OVP_24V,
    LM3533_BOOST_OVP_32V,
    LM3533_BOOST_OVP_40V,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3533_platform_data {
    pub boost_ovp: lm3533_boost_ovp,
    pub boost_freq: lm3533_boost_freq,
    pub als: *mut lm3533_als_platform_data,
    pub backlights: *mut lm3533_bl_platform_data,
    pub num_backlights: c_int,
    pub leds: *mut lm3533_led_platform_data,
    pub num_leds: c_int,
}

extern "C" {
    pub fn lm3533_ctrlbank_enable(cb: *mut lm3533_ctrlbank) -> c_int;
}
extern "C" {
    pub fn lm3533_ctrlbank_disable(cb: *mut lm3533_ctrlbank) -> c_int;
}
extern "C" {
    pub fn lm3533_ctrlbank_set_brightness(cb: *mut lm3533_ctrlbank, val: u8) -> c_int;
}
extern "C" {
    pub fn lm3533_ctrlbank_get_brightness(cb: *mut lm3533_ctrlbank, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn lm3533_ctrlbank_set_pwm(cb: *mut lm3533_ctrlbank, val: u8) -> c_int;
}
extern "C" {
    pub fn lm3533_ctrlbank_get_pwm(cb: *mut lm3533_ctrlbank, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn lm3533_read(lm3533: *mut lm3533, reg: u8, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn lm3533_write(lm3533: *mut lm3533, reg: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn lm3533_update(lm3533: *mut lm3533, reg: u8, val: u8, mask: u8) -> c_int;
}
