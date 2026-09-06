//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/intel/thermal_interrupt.h
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
pub const CORE_LEVEL: c_int = 0;
pub const PACKAGE_LEVEL: c_int = 1;
// Interrupt Handler for package thermal thresholds
extern "C" {
    pub fn int(msr_val: *mut *mut platform_thermal_package_notify)(__u64) -> extern;
}
// Interrupt Handler for core thermal thresholds
extern "C" {
    pub fn int(msr_val: *mut *mut platform_thermal_notify)(__u64) -> extern;
}
// Callback support of rate control, return true, if
// callback has rate control
extern "C" {
    pub fn bool(_arg: *mut platform_thermal_package_rate_control)(void) -> extern;
}
// Handle HWP interrupt
extern "C" {
    pub fn notify_hwp_interrupt();
}
// Common function to clear Package thermal status register
extern "C" {
    pub fn thermal_clear_package_intr_status(level: c_int, bit_mask: u64);
}
