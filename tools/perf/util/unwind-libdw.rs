//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/unwind-libdw.h
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
pub struct unwind_info {
    pub dwfl: *mut c_void,
    pub sample: *mut perf_sample,
    pub machine: *mut machine,
    pub thread: *mut thread,
    pub cb: unwind_entry_cb_t,
    pub arg: *mut c_void,
    pub max_stack: c_int,
    pub idx: c_int,
    pub e_flags: u32,
    pub e_machine: u16,
    pub best_effort: bool,
    pub entries: [unwind_entry; ],
}

extern "C" {
    pub fn libdw__invalidate_dwfl(maps: *mut maps, dwfl: *mut c_void);
}

