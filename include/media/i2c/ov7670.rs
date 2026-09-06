//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/ov7670.h
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
// A V4L2 driver for OmniVision OV7670 cameras.
//
// Copyright 2010 One Laptop Per Child
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov7670_config {
    pub /: *mut *mut int min_width; / Filter out smaller sizes,
    pub /: *mut *mut int min_height; / Filter out smaller sizes,
    pub /: *mut *mut int clock_speed; / External clock speed (MHz),
    pub /: *mut *mut bool use_smbus; / Use smbus I/O instead of I2C,
    pub /: *mut *mut bool pll_bypass; / Choose whether to bypass the PLL,
    pub /: *mut *mut bool pclk_hb_disable; / Disable toggling pixclk during horizontal blanking,
}
