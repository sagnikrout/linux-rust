//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm9081.h
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
// linux/sound/wm9081.h -- Platform data for WM9081
//
// Copyright 2009 Wolfson Microelectronics. PLC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm9081_retune_mobile_setting {
    pub name: *const c_char,
    pub rate: c_uint,
    pub config: [u16; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm9081_pdata {
    pub /: *mut *mut bool irq_high; / IRQ is active high,
    pub /: *mut *mut bool irq_cmos; / IRQ is in CMOS mode,
    pub retune_configs: *mut wm9081_retune_mobile_setting,
    pub num_retune_configs: c_int,
}
