//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/debug.h
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
// Tegra host1x Debug
//
// Copyright (c) 2011-2013 NVIDIA Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct output {
    pub cont): *const *const *const *const void (fn)(void ctx, char str, size_t len, bool,
    pub ctx: *mut c_void,
    pub buf: [c_char; 256],
}

extern "C" {
    pub fn __printf(_arg: 2, o: *mut 3) host1x_debug_output(struct output, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __printf(_arg: 2, o: *mut 3) host1x_debug_cont(struct output, fmt: *const c_char, ...);
}
extern "C" {
    pub fn host1x_debug_init(host1x: *mut host1x);
}
extern "C" {
    pub fn host1x_debug_deinit(host1x: *mut host1x);
}
extern "C" {
    pub fn host1x_debug_dump(host1x: *mut host1x);
}
