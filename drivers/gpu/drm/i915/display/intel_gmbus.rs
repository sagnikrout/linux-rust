//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_gmbus.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

pub const GMBUS_PIN_DISABLED: c_int = 0;
pub const GMBUS_PIN_SSC: c_int = 1;
pub const GMBUS_PIN_VGADDC: c_int = 2;
pub const GMBUS_PIN_PANEL: c_int = 3;

pub const GMBUS_PIN_2: c_int = 2;
pub const GMBUS_PIN_3: c_int = 3;
pub const GMBUS_PIN_4: c_int = 4;
pub const GMBUS_PIN_5: c_int = 5;

pub const GMBUS_PIN_10_TC2: c_int = 10;
pub const GMBUS_PIN_11_TC3: c_int = 11;
pub const GMBUS_PIN_12_TC4: c_int = 12;
pub const GMBUS_PIN_13_TC5: c_int = 13;
pub const GMBUS_PIN_14_TC6: c_int = 14;

extern "C" {
    pub fn intel_gmbus_setup(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_gmbus_teardown(display: *mut intel_display);
}
extern "C" {
    pub fn intel_gmbus_is_valid_pin(display: *mut intel_display, pin: c_uint) -> bool;
}
extern "C" {
    pub fn intel_gmbus_output_aksv(adapter: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn intel_gmbus_force_bit(adapter: *mut i2c_adapter, force_bit: bool);
}
extern "C" {
    pub fn intel_gmbus_is_forced_bit(adapter: *mut i2c_adapter) -> bool;
}
extern "C" {
    pub fn intel_gmbus_reset(display: *mut intel_display);
}
extern "C" {
    pub fn intel_gmbus_irq_handler(display: *mut intel_display);
}
