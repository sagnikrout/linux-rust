//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/print-events.h
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
pub struct print_callbacks {
    pub print_state): *mut *mut void (print_start)(void,
    pub print_state): *mut *mut void (print_end)(void,
    pub encoding_desc): *const c_char,
    pub pmu_name): *const c_char,
    pub print_state): *mut *mut bool (skip_duplicate_pmus)(void,
}

// Print all events, the default when no options are specified.
extern "C" {
    pub fn print_events(print_cb: *const print_callbacks, print_state: *mut c_void);
}
extern "C" {
    pub fn print_sdt_events(print_cb: *const print_callbacks, print_state: *mut c_void);
}
extern "C" {
    pub fn metricgroup__print(print_cb: *const print_callbacks, print_state: *mut c_void);
}
extern "C" {
    pub fn is_event_supported(type: u8, config: u64) -> bool;
}
