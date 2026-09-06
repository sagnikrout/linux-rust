//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/unwind_deferred_types.h
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
pub struct unwind_cache {
    pub unwind_completed: c_ulong,
    pub nr_entries: c_uint,
    pub entries: [c_ulong; ],
}

//
// The unwind_task_id is a unique identifier that maps to a user space
// stacktrace. It is generated the first time a deferred user space
// stacktrace is requested after a task has entered the kerenl and
// is cleared to zero when it exits. The mapped id will be a non-zero
// number.
//
// To simplify the generation of the 64 bit number, 32 bits will be
// the CPU it was generated on, and the other 32 bits will be a per
// cpu counter that gets incremented by two every time a new identifier
// is generated. The LSB will always be set to keep the value
// from being zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union unwind_task_id {
    pub cpu: u32,
    pub cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_task_info {
    pub unwind_mask: atomic_long_t,
    pub cache: *mut unwind_cache,
    pub work: callback_head,
    pub id: unwind_task_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_work {
    pub list: list_head,
    pub func: unwind_callback_t,
    pub bit: c_int,
}
