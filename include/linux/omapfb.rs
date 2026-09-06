//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/omapfb.h
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
// File: include/linux/omapfb.h
//
// Framebuffer driver for TI OMAP boards
//
// Copyright (C) 2004 Nokia Corporation
// Author: Imre Deak <imre.deak@nokia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_lcd_config {
    pub panel_name: [c_char; 16],
    pub ctrl_name: [c_char; 16],
    pub nreset_gpio: i16,
    pub data_lines: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_platform_data {
    pub lcd: omap_lcd_config,
}

extern "C" {
    pub fn omapfb_set_lcd_config(config: *const omap_lcd_config) -> void __init;
}
