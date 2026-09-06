//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/wm8775.h
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
// The WM8775 has 4 inputs and one output. Zero or more inputs
pub const WM8775_AIN1: c_int = 1;
pub const WM8775_AIN2: c_int = 2;
pub const WM8775_AIN3: c_int = 4;
pub const WM8775_AIN4: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8775_platform_data {
//
// FIXME: Instead, we should parameterize the params
// that need different settings between ivtv, pvrusb2, and Nova-S
//
    pub is_nova_s: bool,
}
