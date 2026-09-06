//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm5100.h
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
// linux/sound/wm5100.h -- Platform data for WM5100
//
// Copyright 2011 Wolfson Microelectronics. PLC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm5100_in_mode {
    WM5100_IN_SE = 0,
    WM5100_IN_DIFF = 1,
    WM5100_IN_DMIC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm5100_dmic_sup {
    WM5100_DMIC_SUP_MICVDD = 0,
    WM5100_DMIC_SUP_MICBIAS1 = 1,
    WM5100_DMIC_SUP_MICBIAS2 = 2,
    WM5100_DMIC_SUP_MICBIAS3 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm5100_micdet_bias {
    WM5100_MICDET_MICBIAS1 = 0,
    WM5100_MICDET_MICBIAS2 = 1,
    WM5100_MICDET_MICBIAS3 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm5100_jack_mode {
    pub bias: wm5100_micdet_bias,
    pub hp_pol: c_int,
    pub micd_src: c_int,
}

pub const WM5100_GPIO_SET: c_uint = 0x10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm5100_pdata {
    pub irq_flags: c_int,
    pub jack_modes: [wm5100_jack_mode; 2],
// Input pin mode selection
    pub in_mode: [wm5100_in_mode; 4],
// DMIC supply selection
    pub dmic_sup: [wm5100_dmic_sup; 4],
    pub gpio_defaults: [c_int; 6],
}
