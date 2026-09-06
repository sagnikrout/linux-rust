//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm2200.h
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
// linux/sound/wm2200.h -- Platform data for WM2200
//
// Copyright 2012 Wolfson Microelectronics. PLC.
//
pub const WM2200_GPIO_SET: c_uint = 0x10000;
pub const WM2200_MAX_MICBIAS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm2200_in_mode {
    WM2200_IN_SE = 0,
    WM2200_IN_DIFF = 1,
    WM2200_IN_DMIC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm2200_dmic_sup {
    WM2200_DMIC_SUP_MICVDD = 0,
    WM2200_DMIC_SUP_MICBIAS1 = 1,
    WM2200_DMIC_SUP_MICBIAS2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm2200_mbias_lvl {
    WM2200_MBIAS_LVL_1V5 = 1,
    WM2200_MBIAS_LVL_1V8 = 2,
    WM2200_MBIAS_LVL_1V9 = 3,
    WM2200_MBIAS_LVL_2V0 = 4,
    WM2200_MBIAS_LVL_2V2 = 5,
    WM2200_MBIAS_LVL_2V4 = 6,
    WM2200_MBIAS_LVL_2V5 = 7,
    WM2200_MBIAS_LVL_2V6 = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm2200_micbias {
    pub /: *mut *mut *mut wm2200_mbias_lvl mb_lvl; / Regulated voltage,
    pub /: *mut *mut *mut unsigned int discharge:1; / Actively discharge,
    pub /: *mut *mut *mut unsigned int fast_start:1; / Enable aggressive startup ramp rate,
    pub /: *mut *mut *mut unsigned int bypass:1; / Use bypass mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm2200_pdata {
    pub irq_flags: c_int,
    pub gpio_defaults: [c_int; 4],
    pub in_mode: [wm2200_in_mode; 3],
    pub dmic_sup: [wm2200_dmic_sup; 3],
// MICBIAS configurations
    pub micbias: [wm2200_micbias; WM2200_MAX_MICBIAS],
}
