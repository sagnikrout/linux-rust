//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds-expresswire.h
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
// Shared library for Kinetic's ExpressWire protocol.
// This protocol works by pulsing the ExpressWire IC's control GPIO.
// ktd2692 and ktd2801 are known to use this protocol.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expresswire_timing {
    pub poweroff_us: c_ulong,
    pub detect_delay_us: c_ulong,
    pub detect_us: c_ulong,
    pub data_start_us: c_ulong,
    pub end_of_data_low_us: c_ulong,
    pub end_of_data_high_us: c_ulong,
    pub short_bitset_us: c_ulong,
    pub long_bitset_us: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expresswire_common_props {
    pub ctrl_gpio: *mut gpio_desc,
    pub timing: expresswire_timing,
}

extern "C" {
    pub fn expresswire_power_off(props: *mut expresswire_common_props);
}
extern "C" {
    pub fn expresswire_enable(props: *mut expresswire_common_props);
}
extern "C" {
    pub fn expresswire_write_u8(props: *mut expresswire_common_props, val: u8);
}
