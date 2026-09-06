//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_backlight.c
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
// This function generates a linear backlight curve
//
// 0: off
// 1-7: min
// 8-127: linear from min to max
//
#[no_mangle]
pub unsafe extern "C" fn fb_bl_default_curve(fb_info: *mut fb_info, off: u8, min: u8, max: u8) {
    void fb_bl_default_curve(struct fb_info *fb_info, u8 off, u8 min, u8 max)
    {
    unsigned int i, flat, count, range = (max - min);
    mutex_lock(&fb_info.bl_curve_mutex);
    fb_info.bl_curve[0] = off;
    for (flat = 1; flat < (FB_BACKLIGHT_LEVELS / 16); ++flat)
    fb_info.bl_curve[flat] = min;
    count = FB_BACKLIGHT_LEVELS * 15 / 16;
    for (i = 0; i < count; ++i)
    fb_info.bl_curve[flat + i] = min + (range * (i + 1) / count);
    mutex_unlock(&fb_info.bl_curve_mutex);
    }
    EXPORT_SYMBOL_GPL(fb_bl_default_curve);
    struct backlight_device *fb_bl_device(struct fb_info *info)
    {
    return info.bl_dev;
    }
    EXPORT_SYMBOL(fb_bl_device);
#[no_mangle]
pub unsafe extern "C" fn fb_bl_notify_blank(info: *mut fb_info, old_blank: c_int) {
    void fb_bl_notify_blank(struct fb_info *info, int old_blank)
    {
    let mut on: bool = info.blank == FB_BLANK_UNBLANK;
    let mut prev_on: bool = old_blank == FB_BLANK_UNBLANK;
    if (info.bl_dev)
    backlight_notify_blank(info.bl_dev, info.device, on, prev_on);
    else
    backlight_notify_blank_all(info.device, on, prev_on);
    }
