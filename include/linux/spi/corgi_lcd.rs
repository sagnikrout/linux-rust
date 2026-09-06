//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/corgi_lcd.h
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


// SPDX-License-Identifier: GPL-2.0
pub const CORGI_LCD_MODE_QVGA: c_int = 1;
pub const CORGI_LCD_MODE_VGA: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct corgi_lcd_platform_data {
    pub init_mode: c_int,
    pub max_intensity: c_int,
    pub default_intensity: c_int,
    pub limit_mask: c_int,
    pub intensity): *mut *mut void (notify)(int,
    pub (*kick_battery)(void): *mut c_void,
}

extern "C" {
    pub fn corgi_lcd_limit_intensity(limit: c_int);
}
