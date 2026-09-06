//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm8960.h
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
// wm8960.h  --  WM8960 Soc Audio driver platform data
//
pub const WM8960_DRES_400R: c_int = 0;
pub const WM8960_DRES_200R: c_int = 1;
pub const WM8960_DRES_600R: c_int = 2;
pub const WM8960_DRES_150R: c_int = 3;
pub const WM8960_DRES_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8960_data {
    pub /: *mut *mut bool capless; / Headphone outputs configured in capless mode,
    pub /: *mut *mut bool shared_lrclk; / DAC and ADC LRCLKs are wired together,
//
// Setup for headphone detection
//
// hp_cfg[0]: HPSEL[1:0] of R48 (Additional Control 4)
// hp_cfg[1]: {HPSWEN:HPSWPOL} of R24 (Additional Control 2).
// hp_cfg[2]: {TOCLKSEL:TOEN} of R23 (Additional Control 1).
//
    pub hp_cfg: [u32; 3],
//
// Setup for gpio configuration
//
// gpio_cfg[0]: ALRCGPIO of R9 (Audio interface)
// gpio_cfg[1]: {GPIOPOL:GPIOSEL[2:0]} of R48 (Additional Control 4).
//
    pub gpio_cfg: [u32; 2],
}
