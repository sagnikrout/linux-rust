//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/watchdog/watchdog_pretimeout.h
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
pub const WATCHDOG_GOV_NAME_MAXLEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_governor {
    pub name: [c_char; WATCHDOG_GOV_NAME_MAXLEN],
    pub wdd): *mut *mut void (pretimeout)(struct watchdog_device,
}

// Interfaces to watchdog pretimeout governors
extern "C" {
    pub fn watchdog_register_governor(gov: *mut watchdog_governor) -> c_int;
}
extern "C" {
    pub fn watchdog_unregister_governor(gov: *mut watchdog_governor);
}
// Interfaces to watchdog_dev.c
extern "C" {
    pub fn watchdog_register_pretimeout(wdd: *mut watchdog_device) -> c_int;
}
extern "C" {
    pub fn watchdog_unregister_pretimeout(wdd: *mut watchdog_device);
}
extern "C" {
    pub fn watchdog_pretimeout_available_governors_get(buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn watchdog_pretimeout_governor_get(wdd: *mut watchdog_device, buf: *mut c_char) -> c_int;
}

