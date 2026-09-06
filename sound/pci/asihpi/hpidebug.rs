//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpidebug.h
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

// Define debugging levels.

// an OS can define an extra flag string that is appended to

extern "C" {
    pub fn hpi_debug_init();
}
extern "C" {
    pub fn hpi_debug_level_set(level: c_int) -> c_int;
}
extern "C" {
    pub fn hpi_debug_level_get() -> c_int;
}
// needed by Linux driver for dynamic debug level changes
extern "C" {
    pub fn hpi_debug_message(phm: *mut hpi_message, sz_fileline: *mut c_char);
}
extern "C" {
    pub fn hpi_debug_data(pdata: *mut u16, len: u32);
}

