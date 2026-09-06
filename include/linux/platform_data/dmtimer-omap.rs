//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/dmtimer-omap.h
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
// DMTIMER platform data for TI OMAP platforms
//
// Copyright (C) 2012 Texas Instruments
// Author: Jon Hunter <jon-hunter@ti.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dm_timer_ops {
    pub np): *mut *mut *mut omap_dm_timer (request_by_node)(device_node,
    pub timer_id): *mut *mut *mut omap_dm_timer (request_specific)(int,
    pub (*request)(void): *mut omap_dm_timer,
    pub timer): *mut *mut int (free)(struct omap_dm_timer,
    pub timer): *mut *mut void (enable)(struct omap_dm_timer,
    pub timer): *mut *mut void (disable)(struct omap_dm_timer,
    pub timer): *mut *mut int (get_irq)(struct omap_dm_timer,
    pub value): c_uint,
    pub mask): *mut *mut *mut int (set_int_disable)(struct omap_dm_timer timer, u32,
    pub timer): *mut *mut *mut clk (get_fclk)(omap_dm_timer,
    pub timer): *mut *mut int (start)(struct omap_dm_timer,
    pub timer): *mut *mut int (stop)(struct omap_dm_timer,
    pub source): *mut *mut *mut int (set_source)(struct omap_dm_timer timer, int,
    pub value): *mut *mut *mut int (set_load)(struct omap_dm_timer timer, unsigned int,
    pub match): c_uint,
    pub autoreload): int toggle, int trigger, int,
    pub timer): *mut *mut int (get_pwm_status)(struct omap_dm_timer,
    pub config_period): int autoreload, bool,
    pub timer): *mut *mut int (get_cap_status)(struct omap_dm_timer,
    pub prescaler): *mut *mut *mut int (set_prescaler)(struct omap_dm_timer timer, int,
    pub timer): *mut *mut unsigned int (read_counter)(struct omap_dm_timer,
    pub is_period): *mut *mut *mut unsigned int (read_cap)(struct omap_dm_timer timer, bool,
    pub value): c_uint,
    pub timer): *mut *mut unsigned int (read_status)(struct omap_dm_timer,
    pub value): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmtimer_platform_data {
// set_timer_src - Only used for OMAP1 devices
    pub source): *mut *mut *mut int (set_timer_src)(struct platform_device pdev, int,
    pub timer_capability: u32,
    pub timer_errata: u32,
    pub ): *mut *mut int (get_context_loss_count)(struct device,
    pub timer_ops: *const omap_dm_timer_ops,
}
