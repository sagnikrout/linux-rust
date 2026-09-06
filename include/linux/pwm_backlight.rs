//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pwm_backlight.h
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
//
// Generic PWM backlight driver data - see drivers/video/backlight/pwm_bl.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_pwm_backlight_data {
    pub max_brightness: c_uint,
    pub dft_brightness: c_uint,
    pub lth_brightness: c_uint,
    pub pwm_period_ns: c_uint,
    pub levels: *mut c_uint,
    pub post_pwm_on_delay: c_uint,
    pub pwm_off_delay: c_uint,
    pub dev): *mut *mut int (init)(struct device,
    pub brightness): *mut *mut *mut int (notify)(struct device dev, int,
    pub brightness): *mut *mut *mut void (notify_after)(struct device dev, int,
    pub dev): *mut *mut void (exit)(struct device,
}
