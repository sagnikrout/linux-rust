//! Automatically rewritten from C Header to Rust Module
//! Source: include/clocksource/timer-davinci.h
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
// TI DaVinci clocksource driver
//
// Copyright (C) 2019 Texas Instruments
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//

//
// struct davinci_timer_cfg - davinci clocksource driver configuration struct
// @reg:        register range resource
// @irq:        clockevent and clocksource interrupt resources
// @cmp_off:    if set - it specifies the compare register used for clockevent
//
// Note: if the compare register is specified, the driver will use the bottom
// clock half for both clocksource and clockevent and the compare register
// to generate event irqs. The user must supply the correct compare register
// interrupt number.
//
// This is only used by da830 the DSP of which uses the top half. The timer
// driver still configures the top half to run in free-run mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_timer_cfg {
    pub reg: resource,
    pub irq: [resource; DAVINCI_TIMER_NUM_IRQS],
    pub cmp_off: c_uint,
}
