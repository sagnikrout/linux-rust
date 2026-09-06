//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds-lp3944.h
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
// leds-lp3944.h - platform data structure for lp3944 led controller
//
// Copyright (C) 2009 Antonio Ospite <ospite@studenti.unina.it>
//
pub const LP3944_LED0: c_int = 0;
pub const LP3944_LED1: c_int = 1;
pub const LP3944_LED2: c_int = 2;
pub const LP3944_LED3: c_int = 3;
pub const LP3944_LED4: c_int = 4;
pub const LP3944_LED5: c_int = 5;
pub const LP3944_LED6: c_int = 6;
pub const LP3944_LED7: c_int = 7;
pub const LP3944_LEDS_MAX: c_int = 8;
pub const LP3944_LED_STATUS_MASK: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3944_status {
    LP3944_LED_STATUS_OFF  = 0x0,
    LP3944_LED_STATUS_ON   = 0x1,
    LP3944_LED_STATUS_DIM0 = 0x2,
    LP3944_LED_STATUS_DIM1 = 0x3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3944_type {
    LP3944_LED_TYPE_NONE,
    LP3944_LED_TYPE_LED,
    LP3944_LED_TYPE_LED_INVERTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3944_led {
    pub name: *mut c_char,
    pub type: lp3944_type,
    pub status: lp3944_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3944_platform_data {
    pub leds: [lp3944_led; LP3944_LEDS_MAX],
    pub leds_size: u8,
}
