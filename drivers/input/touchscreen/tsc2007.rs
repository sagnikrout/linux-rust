//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/touchscreen/tsc2007.h
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
// Copyright (c) 2008 MtekVision Co., Ltd.
// Kwangwoo Lee <kwlee@mtekvision.com>
//
// Using code from:
// - ads7846.c
// Copyright (c) 2005 David Brownell
// Copyright (c) 2006 Nokia Corporation
// - corgi_ts.c
// Copyright (C) 2004-2005 Richard Purdie
// - omap_ts.[hc], ads7846.h, ts_osk.c
// Copyright (C) 2002 MontaVista Software
// Copyright (C) 2004 Texas Instruments
// Copyright (C) 2005 Dirk Behme
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_event {
    pub x: u16,
    pub y: u16,
    pub z2: u16 z1,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsc2007 {
    pub input: *mut input_dev,
    pub phys: [c_char; 32],
    pub client: *mut i2c_client,
    pub prop: touchscreen_properties,
    pub model: u16,
    pub x_plate_ohms: u16,
    pub max_rt: u16,
    pub /: *mut *mut unsigned long poll_period; / in jiffies,
    pub fuzzx: c_int,
    pub fuzzy: c_int,
    pub fuzzz: c_int,
    pub gpiod: *mut gpio_desc,
    pub irq: c_int,
    pub wait: wait_queue_head_t,
    pub stopped: bool,
    pub ): *mut *mut int (get_pendown_state)(struct device,
    pub (*clear_penirq)(void): *mut c_void,
    pub mlock: mutex,
}

extern "C" {
    pub fn tsc2007_xfer(tsc: *mut tsc2007, cmd: u8) -> c_int;
}
extern "C" {
    pub fn tsc2007_calculate_resistance(tsc: *mut tsc2007, tc: *mut ts_event) -> u32;
}
extern "C" {
    pub fn tsc2007_is_pen_down(ts: *mut tsc2007) -> bool;
}

// defined in tsc2007_iio.c
extern "C" {
    pub fn tsc2007_iio_configure(ts: *mut tsc2007) -> c_int;
}

