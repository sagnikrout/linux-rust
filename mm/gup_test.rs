//! Automatically rewritten from C Header to Rust Module
//! Source: mm/gup_test.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

pub const GUP_TEST_MAX_PAGES_TO_DUMP: c_int = 8;
pub const GUP_TEST_FLAG_DUMP_PAGES_USE_PIN: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gup_test {
    pub get_delta_usec: __u64,
    pub put_delta_usec: __u64,
    pub addr: __u64,
    pub size: __u64,
    pub nr_pages_per_call: __u32,
    pub gup_flags: __u32,
    pub test_flags: __u32,
//
// Each non-zero entry is the number of the page (1-based: first page is
// page 1, so that zero entries mean "do nothing") from the .addr base.
//
    pub which_pages: [__u32; GUP_TEST_MAX_PAGES_TO_DUMP],
}

pub const PIN_LONGTERM_TEST_FLAG_USE_WRITE: c_int = 1;
pub const PIN_LONGTERM_TEST_FLAG_USE_FAST: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pin_longterm_test {
    pub addr: __u64,
    pub size: __u64,
    pub flags: __u32,
}
