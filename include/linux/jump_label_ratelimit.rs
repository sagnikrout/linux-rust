//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/jump_label_ratelimit.h
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
pub struct static_key_deferred {
    pub key: static_key,
    pub timeout: c_ulong,
    pub work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_true_deferred {
    pub key: static_key_true,
    pub timeout: c_ulong,
    pub work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_false_deferred {
    pub key: static_key_false,
    pub timeout: c_ulong,
    pub work: delayed_work,
}

extern "C" {
    pub fn __static_key_deferred_flush(key: *mut c_void, work: *mut delayed_work);
}
extern "C" {
    pub fn jump_label_update_timeout(work: *mut work_struct);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_deferred {
    pub key: static_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_true_deferred {
    pub key: static_key_true,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_false_deferred {
    pub key: static_key_false,
}

