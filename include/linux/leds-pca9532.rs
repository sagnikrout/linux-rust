//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds-pca9532.h
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
// pca9532.h - platform data structure for pca9532 led controller
//
// Copyright (C) 2008 Riku Voipio <riku.voipio@movial.fi>
//
// Datasheet: http://www.nxp.com/acrobat/datasheets/PCA9532_3.pdf
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pca9532_state {
    PCA9532_OFF  = 0x0,
    PCA9532_ON   = 0x1,
    PCA9532_PWM0 = 0x2,
    PCA9532_PWM1 = 0x3,
    PCA9532_KEEP = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca9532_led {
    pub id: u8,
    pub client: *mut i2c_client,
    pub name: *const c_char,
    pub default_trigger: *const c_char,
    pub ldev: led_classdev,
    pub work: work_struct,
    pub type: u32,
    pub state: pca9532_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca9532_platform_data {
    pub leds: [pca9532_led; 16],
    pub pwm: [u8; 2],
    pub psc: [u8; 2],
    pub gpio_base: c_int,
}
