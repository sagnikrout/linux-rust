//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/thermal_thresholds.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_threshold {
    pub list_node: list_head,
    pub temperature: c_int,
    pub direction: c_int,
}

extern "C" {
    pub fn thermal_thresholds_init(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_thresholds_exit(tz: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_thresholds_handle(tz: *mut thermal_zone_device, low: *mut c_int, high: *mut c_int);
}
extern "C" {
    pub fn thermal_thresholds_flush(tz: *mut thermal_zone_device);
}
extern "C" {
    pub fn thermal_thresholds_add(tz: *mut thermal_zone_device, temperature: c_int, direction: c_int) -> c_int;
}
extern "C" {
    pub fn thermal_thresholds_delete(tz: *mut thermal_zone_device, temperature: c_int, direction: c_int) -> c_int;
}
