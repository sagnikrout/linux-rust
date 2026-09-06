//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page_reporting.h
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

pub const PAGE_REPORTING_CAPACITY: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_reporting_dev_info {
// function that alters pages to make them "reported"
    pub nents): *mut *mut scatterlist sg, unsigned int,
// work struct for processing reports
    pub work: delayed_work,
// Current state of page reporting
    pub state: core::sync::atomic::AtomicI32,
// Minimal order of page reporting
    pub order: c_uint,
// Max pages per report batch; 0 (default) means PAGE_REPORTING_CAPACITY
    pub capacity: c_uint,
}

// Tear-down and bring-up for page reporting devices
extern "C" {
    pub fn page_reporting_unregister(prdev: *mut page_reporting_dev_info);
}
extern "C" {
    pub fn page_reporting_register(prdev: *mut page_reporting_dev_info) -> c_int;
}
