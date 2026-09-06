//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/lp855x.h
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
// LP855x Backlight Driver
//
// Copyright (C) 2011 Texas Instruments
//

// Enable backlight. Only valid when BRT_MODE=10(I2C only)

// DEVICE CONTROL register - LP8550

// DEVICE CONTROL register - LP8551

// DEVICE CONTROL register - LP8552

// DEVICE CONTROL register - LP8553

// CONFIG register - LP8555

// DEVICE CONTROL register - LP8556

// CONFIG register - LP8557

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp855x_chip_id {
    LP8550,
    LP8551,
    LP8552,
    LP8553,
    LP8555,
    LP8556,
    LP8557,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8550_brighntess_source {
    LP8550_PWM_ONLY,
    LP8550_I2C_ONLY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8551_brighntess_source {
    LP8551_PWM_ONLY = LP8550_PWM_ONLY,
    LP8551_I2C_ONLY = LP8550_I2C_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8552_brighntess_source {
    LP8552_PWM_ONLY = LP8550_PWM_ONLY,
    LP8552_I2C_ONLY = LP8550_I2C_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8553_brighntess_source {
    LP8553_PWM_ONLY = LP8550_PWM_ONLY,
    LP8553_I2C_ONLY = LP8550_I2C_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8555_brightness_source {
    LP8555_PWM_ONLY,
    LP8555_I2C_ONLY,
    LP8555_COMBINED1,	/* Brightness register with shaped PWM */
    LP8555_COMBINED2,	/* PWM with shaped brightness register */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8556_brightness_source {
    LP8556_PWM_ONLY,
    LP8556_COMBINED1,	/* pwm + i2c before the shaper block */
    LP8556_I2C_ONLY,
    LP8556_COMBINED2,	/* pwm + i2c after the shaper block */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8557_brightness_source {
    LP8557_PWM_ONLY,
    LP8557_I2C_ONLY,
    LP8557_COMBINED1,	/* pwm + i2c after the shaper block */
    LP8557_COMBINED2,	/* pwm + i2c before the shaper block */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp855x_rom_data {
    pub addr: u8,
    pub val: u8,
}

//
// struct lp855x_platform_data - lp855 platform-specific data
// @name : Backlight driver name. If it is not defined, default name is set.
// @device_control : value of DEVICE CONTROL register
// @initial_brightness : initial value of backlight brightness
// @period_ns : platform specific pwm period value. unit is nano.
// Only valid when mode is PWM_BASED.
// @size_program : total size of lp855x_rom_data
// @rom_data : list of new eeprom/eprom registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp855x_platform_data {
    pub name: *const c_char,
    pub device_control: u8,
    pub initial_brightness: u8,
    pub period_ns: c_uint,
    pub size_program: c_int,
    pub rom_data: *mut lp855x_rom_data,
}
