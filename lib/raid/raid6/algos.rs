//! Automatically rewritten from C Header to Rust Module
//! Source: lib/raid/raid6/algos.h
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
//
// Copyright 2003 H. Peter Anvin - All Rights Reserved
//

// Routine choices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid6_calls {
    pub name: *const c_char,
    pub ptrs): *mut *mut void (gen_syndrome)(int disks, size_t bytes, void,
    pub ptrs): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid6_recov_calls {
    pub name: *const c_char,
    pub ptrs): *mut c_void,
    pub ptrs): *mut *mut void (datap)(int disks, size_t bytes, int faila, void,
}

extern "C" {
    pub fn raid6_algo_add(algo: *const raid6_calls) -> void __init;
}
extern "C" {
    pub fn raid6_algo_add_default() -> void __init;
}
extern "C" {
    pub fn raid6_recov_algo_add(algo: *const raid6_recov_calls) -> void __init;
}
// for the kunit test
// generic implementations
