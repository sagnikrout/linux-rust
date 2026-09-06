//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/ads7846.h
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


// SPDX-License-Identifier: GPL-2.0
// linux/spi/ads7846.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ads7846_platform_data {
    pub /: *mut *mut u16 model; / 7843, 7845, 7846, 7873.,
    pub /: *mut *mut u16 vref_delay_usecs; / 0 for external vref; etc,
    pub milliVolts: *mut *mut u16 vref_mv; / external vref value,,
// ads7846: if 0, use internal vref
    pub differential: *mut *mut bool keep_vref_on; / set to keep vref on for,
// measurements as well
    pub /: *mut *mut bool swap_xy; / swap x and y axes,
// Settling time of the analog signals; a function of Vcc and the
// capacitance on the X/Y drivers.  If set to non-zero, two samples
// are taken with settle_delay us apart, and the second one is used.
// ~150 uSec with 0.01uF caps.
//
    pub settle_delay_usecs: u16,
// If set to non-zero, after samples are taken this delay is applied
// and penirq is rechecked, to help avoid false events.  This value
// is affected by the material used to build the touch layer.
//
    pub penirq_recheck_delay_usecs: u16,
    pub x_plate_ohms: u16,
    pub y_plate_ohms: u16,
    pub x_max: u16 x_min,,
    pub y_max: u16 y_min,,
    pub pressure_max: u16 pressure_min,,
    pub readings: *mut *mut u16 debounce_max; / max number of additional,
// per sample
    pub /: *mut *mut u16 debounce_tol; / tolerance used for filtering,
    pub readings: *mut *mut u16 debounce_rep; / additional consecutive good,
// required after the first two
    pub for: *mut *mut int gpio_pendown_debounce; / platform specific debounce time,
// the gpio_pendown
    pub (*get_pendown_state)(void): *mut c_int,
    pub (*wait_for_sync)(void): *mut c_void,
    pub wakeup: bool,
    pub irq_flags: c_ulong,
}
