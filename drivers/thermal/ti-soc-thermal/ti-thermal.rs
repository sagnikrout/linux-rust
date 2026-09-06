//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/ti-soc-thermal/ti-thermal.h
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
// OMAP thermal definitions
//
// Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com
// Contact:
// Eduardo Valentin <eduardo.valentin@ti.com>
//

// PCB sensor calculation constants
pub const OMAP_GRADIENT_SLOPE_W_PCB_4430: c_int = 0;
pub const OMAP_GRADIENT_CONST_W_PCB_4430: c_int = 20000;
pub const OMAP_GRADIENT_SLOPE_W_PCB_4460: c_int = 1142;

pub const OMAP_GRADIENT_SLOPE_W_PCB_4470: c_int = 1063;

pub const OMAP_GRADIENT_SLOPE_W_PCB_5430_CPU: c_int = 100;
pub const OMAP_GRADIENT_CONST_W_PCB_5430_CPU: c_int = 484;
pub const OMAP_GRADIENT_SLOPE_W_PCB_5430_GPU: c_int = 464;

pub const DRA752_GRADIENT_SLOPE_W_PCB: c_int = 0;
pub const DRA752_GRADIENT_CONST_W_PCB: c_int = 2000;
// trip points of interest in milicelsius (at hotspot level)
pub const OMAP_TRIP_COLD: c_int = 100000;
pub const OMAP_TRIP_HOT: c_int = 110000;
pub const OMAP_TRIP_SHUTDOWN: c_int = 125000;
pub const OMAP_TRIP_NUMBER: c_int = 2;

// Update rates
pub const FAST_TEMP_MONITORING_RATE: c_int = 250;

extern "C" {
    pub fn ti_thermal_expose_sensor(bgp: *mut ti_bandgap, id: c_int, domain: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ti_thermal_remove_sensor(bgp: *mut ti_bandgap, id: c_int) -> c_int;
}
extern "C" {
    pub fn ti_thermal_report_sensor_temperature(bgp: *mut ti_bandgap, id: c_int) -> c_int;
}
extern "C" {
    pub fn ti_thermal_register_cpu_cooling(bgp: *mut ti_bandgap, id: c_int) -> c_int;
}
extern "C" {
    pub fn ti_thermal_unregister_cpu_cooling(bgp: *mut ti_bandgap, id: c_int) -> c_int;
}

