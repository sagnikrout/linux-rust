//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_utils.h
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


// SPDX-License-Identifier: MIT
//
// Function prototypes for misc. drm utility functions.
// Specifically this file is for function prototypes for functions which
// may also be used outside of drm code (e.g. in fbdev drivers).
//
// Copyright (C) 2017 Hans de Goede <hdegoede@redhat.com>
//

extern "C" {
    pub fn drm_get_panel_orientation_quirk(width: c_int, height: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panel_backlight_quirk {
    pub min_brightness: u16,
    pub brightness_mask: u32,
    pub force_pwm: bool,
}

extern "C" {
    pub fn drm_timeout_abs_to_jiffies(timeout_nsec: i64) -> signed long;
}
