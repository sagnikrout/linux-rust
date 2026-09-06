//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dw_apb_timer.h
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
// (C) Copyright 2009 Intel Corporation
// Author: Jacob Pan (jacob.jun.pan@intel.com)
//
// Shared with ARM platforms, Jamie Iles, Picochip 2011
//
// Support for the Synopsys DesignWare APB Timers.
//

pub const APBTMRS_REG_SIZE: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_apb_timer {
    pub base: *mut void __iomem,
    pub freq: c_ulong,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_apb_clock_event_device {
    pub ced: clock_event_device,
    pub timer: dw_apb_timer,
    pub ): *mut *mut void (eoi)(struct dw_apb_timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_apb_clocksource {
    pub timer: dw_apb_timer,
    pub cs: clocksource,
}

extern "C" {
    pub fn dw_apb_clockevent_register(dw_ced: *mut dw_apb_clock_event_device);
}
extern "C" {
    pub fn dw_apb_clocksource_register(dw_cs: *mut dw_apb_clocksource);
}
extern "C" {
    pub fn dw_apb_clocksource_start(dw_cs: *mut dw_apb_clocksource);
}
extern "C" {
    pub fn dw_apb_clocksource_read(dw_cs: *mut dw_apb_clocksource) -> u64;
}
