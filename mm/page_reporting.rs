//! Automatically rewritten from C Header to Rust Module
//! Source: mm/page_reporting.h
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

extern "C" {
    pub fn __page_reporting_notify();
}
//
// page_reporting_notify_free - Free page notification to start page processing
//
// This function is meant to act as a screener for __page_reporting_notify
// which will determine if a give zone has crossed over the high-water mark
// that will justify us beginning page treatment. If we have crossed that
// threshold then it will start the process of pulling some pages and
// placing them in the batch list for treatment.
//
// Called from hot path in __free_one_page()
// Determine if we have crossed reporting threshold
// This will add a few cycles, but should be called infrequently

