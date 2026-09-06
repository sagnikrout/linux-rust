//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/acpi_pmtmr.h
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

// Number of PMTMR ticks expected during calibration run
pub const PMTMR_TICKS_PER_SEC: c_int = 3579545;
// limit it to 24 bits

// Overrun value

extern "C" {
    pub fn acpi_pm_read_verified() -> u32;
}
// mask the output to 24 bits
//
// acpi_pmtmr_register_suspend_resume_callback - Register callback for
// suspend and resume event
//
// @cb: Callback triggered on suspend and resume
// @data: Data passed with the callback
//
extern "C" {
    pub fn acpi_pmtmr_register_suspend_resume_callback(data: *mut *mut void (cb)(void, suspend): bool, data: *mut c_void);
}
//
// acpi_pmtmr_unregister_suspend_resume_callback - Remove registered callback
// for suspend and resume event
//
extern "C" {
    pub fn acpi_pmtmr_unregister_suspend_resume_callback();
}

